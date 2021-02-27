use crate::{
    constraint::Constraint,
    error::{TyError, TyResult},
    inference::InferTy,
    substitution::Substitution,
    table::{Table, TableBuilder},
    unifier::{Unifier, UnifierBuilder},
};

use pijama_hir::{FuncId, Local, Name, Program};
use pijama_ty::{
    inference::{Ty, TyContext},
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
    fn update_constraints(&mut self, subst: &Substitution) {
        for Constraint { lhs, rhs } in &mut self.constraints {
            subst.apply_to(lhs);
            subst.apply_to(rhs);
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
                (Ty::Hole(id), rhs) if !rhs.contains_hole(id) => {
                    let subs = Substitution::new(id, rhs);
                    // Replace lhs by rhs in all the constraints.
                    self.update_constraints(&subs);
                    // Keep unifying.
                    self.unify(builder)?;
                    // Add this substitution to the builder.
                    builder.add_substitution(subs);
                }
                // If the right-hand side type is a free variable in the left-hand side, we can
                // replace the right-hand side by the left-hand side.
                (lhs, Ty::Hole(id)) if !lhs.contains_hole(id) => {
                    let subs = Substitution::new(id, lhs);
                    // Replace rhs by lhs in all the constraints.
                    self.update_constraints(&subs);
                    // Keep unifying.
                    self.unify(builder)?;
                    // Add this substitution to the builder.
                    builder.add_substitution(subs);
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
                // If both sides are tuples. Unify each type inside them recursively.
                (Ty::Tuple { fields: fields_ty1 }, Ty::Tuple { fields: fields_ty2 }) => {
                    // Error if the lengths of the tuples do not match.
                    if fields_ty1.len() != fields_ty2.len() {
                        // FIXME: Technically this is not an arity mismatch
                        return Err(TyError::ArityMismatch {
                            expected: fields_ty1.len(),
                            found: fields_ty2.len(),
                        });
                    }

                    // The types of the fields must be equal one-to-one.
                    for (lhs, rhs) in fields_ty1.into_iter().zip(fields_ty2.into_iter()) {
                        self.add_constraint(lhs, rhs);
                    }

                    // Keep unifying.
                    self.unify(builder)?;
                }
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
