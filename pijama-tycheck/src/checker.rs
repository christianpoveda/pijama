use crate::{
    constraint::Constraint,
    error::{TyError, TyResult},
    inference::InferTy,
    substitution::{RowSubstitution, TySubstitution},
    table::{Table, TableBuilder},
    unifier::{Unifier, UnifierBuilder},
};

use pijama_hir::{FuncId, Local, Name, Program};
use pijama_ty::{
    inference::{Row, Ty, TyContext},
    ExprId,
};
use pijama_utils::index::IndexMap;

use std::collections::VecDeque;

pub(crate) struct Checker<'tcx> {
    /// The typing context.
    pub(crate) tcx: &'tcx TyContext,
    /// The types of the locals of the function being type-checked.
    locals_ty: IndexMap<Local, Ty>,
    /// The types of all the functions in the program.
    funcs_ty: IndexMap<FuncId, Ty>,
    /// The set of constraints that the program must satisfy to be well-typed.
    constraints: VecDeque<Constraint>,
    table: TableBuilder,
}

impl<'tcx> Checker<'tcx> {
    /// Return a new checker.
    pub(crate) fn new(tcx: &'tcx TyContext) -> Self {
        Self {
            tcx,
            locals_ty: IndexMap::new(),
            funcs_ty: IndexMap::new(),
            constraints: VecDeque::new(),
            table: Table::builder(tcx.count_expr_ids()),
        }
    }

    /// Type-check a program, consuming the checker in the process. If the type-checking was
    /// successful, return an [Unifier] to instantiate all the type variables.
    pub(crate) fn check_program(mut self, program: &Program) -> TyResult<(Unifier, Table)> {
        // Reconstruct the type of each function in the program.
        let funcs_ty = program
            .functions
            .iter()
            .map(|(_, func)| Ty::Func {
                params_ty: func
                    .locals
                    .iter()
                    .take(func.arity)
                    .map(|(_, ty)| ty.clone())
                    .collect(),
                return_ty: Box::new(func.return_ty.clone()),
            })
            .collect();
        // FIXME: maybe it is better to do this during initialization?.
        // Put the types of the function into the checker.
        self.funcs_ty = IndexMap::from_raw(funcs_ty);

        // Type-check every function.
        for (_, func) in &program.functions {
            // Put the types of the locals in the checker.
            self.locals_ty = func.locals.clone();
            // Infer the type of the body of the function.
            let body_ty = func.body.infer_ty(&mut self)?;
            // The type of the body must be equal to the return type of the function.
            self.add_constraint(func.return_ty.clone(), body_ty);
        }

        // Unify all the constraints.
        let mut builder = Unifier::builder();
        self.unify(&mut builder)?;

        // Build an unifier.
        let unifier = builder.build()?;

        let table = self.table.build(&unifier).unwrap();

        Ok((unifier, table))
    }

    /// Get the type of a name.
    pub(crate) fn get_name_ty(&self, name: &Name) -> Option<&Ty> {
        match name {
            Name::Local(local) => self.locals_ty.get(*local),
            Name::FuncPtr(func_id) => self.funcs_ty.get(*func_id),
        }
    }

    /// Add a constraint that the program must satisfy to be well-typed.
    pub(crate) fn add_constraint(&mut self, lhs: Ty, rhs: Ty) {
        self.constraints.push_front(Constraint::new(lhs, rhs));
    }

    /// Apply a substitution to all the remaining constraints.
    fn update_constraints_ty(&mut self, subst: &TySubstitution) {
        for Constraint { lhs, rhs } in &mut self.constraints {
            subst.apply_to_ty(lhs);
            subst.apply_to_ty(rhs);
        }
    }

    fn update_constraints_row(&mut self, subst: &RowSubstitution) {
        for Constraint { lhs, rhs } in &mut self.constraints {
            subst.apply_to_ty(lhs);
            subst.apply_to_ty(rhs);
        }
    }

    /// Unify the set of constraints.
    ///
    /// If this function runs successfully, the `substitutions` field can be used to substitute any
    /// type.
    fn unify(&mut self, builder: &mut UnifierBuilder) -> TyResult {
        // FIXME: check if it is better to pop from the other end.
        // Keep unifying while there are constraints to unify.
        if let Some(Constraint { lhs, rhs }) = self.constraints.pop_back() {
            // Skip the constraint if both sides of the constraint are equal.
            if lhs == rhs {
                return self.unify(builder);
            }

            match (lhs, rhs) {
                // If the left-hand side type is a free variable in the right-hand side, we can
                // replace the left-hand side by the right-hand side.
                (Ty::Var(id), rhs) if !rhs.contains_ty(id) => {
                    let subs = TySubstitution::new(id, rhs);
                    // Replace lhs by rhs in all the constraints.
                    self.update_constraints_ty(&subs);
                    // Keep unifying.
                    self.unify(builder)?;
                    // Add this substitution to the builder.
                    builder.add_ty_substitution(subs);
                }
                // If the right-hand side type is a free variable in the left-hand side, we can
                // replace the right-hand side by the left-hand side.
                (lhs, Ty::Var(id)) if !lhs.contains_ty(id) => {
                    let subs = TySubstitution::new(id, lhs);
                    // Replace rhs by lhs in all the constraints.
                    self.update_constraints_ty(&subs);
                    // Keep unifying.
                    self.unify(builder)?;
                    // Add this substitution to the builder.
                    builder.add_ty_substitution(subs);
                }
                // If both sides are functions. Unify each type inside them recursively.
                (
                    Ty::Func {
                        params_ty: params_ty1,
                        return_ty: return_ty1,
                    },
                    Ty::Func {
                        params_ty: params_ty2,
                        return_ty: return_ty2,
                    },
                ) => {
                    // Error if the arities of the functions do not match.
                    if params_ty1.len() != params_ty2.len() {
                        return Err(TyError::ArityMismatch {
                            expected: params_ty1.len(),
                            found: params_ty2.len(),
                        });
                    }

                    // The parameters must be equal one-to-one.
                    for (lhs, rhs) in params_ty1.into_iter().zip(params_ty2.into_iter()) {
                        self.add_constraint(lhs, rhs);
                    }

                    // The return types must be equal.
                    self.add_constraint(*return_ty1, *return_ty2);

                    // Keep unifying.
                    self.unify(builder)?;
                }

                (Ty::Record(mut row1), Ty::Record(mut row2)) => match row1.tail() {
                    Some(tail) if row1.fields().is_empty() => {
                        if row2.contains_row(tail) {
                            panic!()
                        }
                        let subs = RowSubstitution::new(tail, row2);
                        self.update_constraints_row(&subs);
                        self.unify(builder)?;
                        builder.add_row_substitution(subs);
                    }
                    _ => match (row1.tail(), row2.tail()) {
                        (None, None) => {
                            let fields1 = row1.fields();
                            let fields2 = row2.fields();

                            if fields1.len() != fields2.len() {
                                panic!()
                            }

                            for ((label1, ty1), (label2, ty2)) in fields1.iter().zip(fields2) {
                                if label1 != label2 {
                                    panic!()
                                }

                                self.add_constraint(ty1.clone(), ty2.clone());
                            }

                            self.unify(builder)?;
                        }
                        (None, Some(tail)) => {
                            while let Some((label, ty2)) = row2.pop_field() {
                                if let Some(ty1) = row1.remove_field(label) {
                                    self.add_constraint(ty1, ty2)
                                } else {
                                    panic!()
                                }
                            }

                            self.add_constraint(
                                Ty::Record(Row::relaxed(vec![], tail)),
                                Ty::Record(row1),
                            );

                            self.unify(builder)?;
                        }
                        (Some(_), None) => unreachable!(),
                        (Some(tail1), Some(tail2)) => {
                            let mut common = Vec::new();

                            for (label, ty1) in row1.fields() {
                                let label = *label;

                                if let Some(ty2) = row2.get_ty(label) {
                                    common.push(label);
                                    self.add_constraint(ty1.clone(), ty2.clone())
                                }
                            }

                            for label in common {
                                row1.remove_field(label);
                                row2.remove_field(label);
                            }

                            let tail = self.tcx.new_row();

                            if row1.contains_row(tail2) {
                                panic!()
                            } else {
                                row1.set_tail(Some(tail));

                                let subs = RowSubstitution::new(tail2, row1);
                                self.update_constraints_row(&subs);
                                self.unify(builder)?;
                            }

                            if row2.contains_row(tail1) {
                                panic!()
                            } else {
                                row2.set_tail(Some(tail));

                                let subs = RowSubstitution::new(tail1, row2);
                                self.update_constraints_row(&subs);
                                self.unify(builder)?;
                            }
                        }
                    },
                },
                // Otherwise, the constraint cannot be satisified.
                (expected, found) => return Err(TyError::TypeMismatch { expected, found }),
            }
        }
        Ok(())
    }

    pub(crate) fn store_ty(&mut self, expr_id: ExprId, ty: Ty) {
        self.table.store_ty(expr_id, ty);
    }
}
