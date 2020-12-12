use crate::{checker::Checker, error::TyResult, inference::InferTy};

use pijama_hir::{Expr, PrimOp};
use pijama_ty::{base::BaseTy, inference::Ty};

impl InferTy for Expr {
    fn infer_ty(&self, checker: &mut Checker) -> TyResult<Ty> {
        match self {
            // Infering the type of an atom is straightforward.
            Expr::Atom(atom) => atom.infer_ty(checker),
            Expr::Let { lhs, rhs, body } => {
                // Infer the types of both sides.
                let lhs_ty = lhs.infer_ty(checker)?;
                let rhs_ty = rhs.infer_ty(checker)?;

                // Those types have to be equal.
                checker.add_constraint(lhs_ty, rhs_ty);

                // Then the type of this expression is the type of the body.
                body.infer_ty(checker)
            }
            Expr::Call { func, args } => {
                // FIXME: Figure out if doing an special case when `func` has a function type is
                // better or not.

                // Infer the type of the called function.
                let lhs_ty = checker.get_name_ty(func).unwrap().clone();

                // Infer the type of ech argument of the call.
                let params_ty = args
                    .iter()
                    .map(|arg| arg.infer_ty(checker))
                    .collect::<TyResult<Vec<Ty>>>()?;

                // Create a new hole for the return type.
                let return_ty = checker.tcx.new_hole();

                // This is the type the function would have if the arguments were well-typed.
                let rhs_ty = Ty::Func {
                    params_ty,
                    return_ty: Box::new(return_ty.clone()),
                };

                // The type of the called function must be equal to the type we just created.
                checker.add_constraint(lhs_ty, rhs_ty);

                // The type of a call is the return type of the function.
                Ok(return_ty)
            }
            Expr::PrimitiveOp { prim_op, ops } => {
                // Define the type the operands must have and the type that the operator returns.
                let (expected_ty, infered_ty) = match prim_op {
                    PrimOp::Add
                    | PrimOp::Sub
                    | PrimOp::Mul
                    | PrimOp::Div
                    | PrimOp::Rem
                    | PrimOp::Neg => {
                        // Arithmetic operators receive integers and return integers.
                        (Ty::Base(BaseTy::Integer), Ty::Base(BaseTy::Integer))
                    }
                    PrimOp::And | PrimOp::Or | PrimOp::Not => {
                        // Logic operators receive booleans and return booleans.
                        (Ty::Base(BaseTy::Bool), Ty::Base(BaseTy::Bool))
                    }
                    PrimOp::Eq | PrimOp::Neq => {
                        // Equality operators receive any type and return booleans.
                        (checker.tcx.new_hole(), Ty::Base(BaseTy::Bool))
                    }
                    PrimOp::Lt | PrimOp::Gt | PrimOp::Lte | PrimOp::Gte => {
                        // Comparison operators receive integers and return booleans.
                        (Ty::Base(BaseTy::Integer), Ty::Base(BaseTy::Bool))
                    }
                };

                for op in ops {
                    // All the operands must have the type that the operator expects.
                    let ty = op.infer_ty(checker)?;
                    checker.add_constraint(expected_ty.clone(), ty);
                }

                // THe type of this expression is the type that the operator returns.
                Ok(infered_ty)
            }
            Expr::Cond {
                cond,
                do_branch,
                else_branch,
            } => {
                let cond = cond.infer_ty(checker)?;
                let do_ty = do_branch.infer_ty(checker)?;
                let else_ty = else_branch.infer_ty(checker)?;

                // The type of the condition must be boolean.
                checker.add_constraint(Ty::Base(BaseTy::Bool), cond);

                // The type of both branches must be the same.
                checker.add_constraint(do_ty.clone(), else_ty);

                // The type of this expression is the type of the branches.
                Ok(do_ty)
            }
        }
    }
}
