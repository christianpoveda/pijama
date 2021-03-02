use crate::{checker::Checker, error::TyResult, inference::InferTy};

use pijama_hir::{BinOp, Expr, ExprKind, UnOp};
use pijama_ty::{base::BaseTy, inference::Ty};

impl InferTy for Expr {
    fn infer_ty(&self, checker: &mut Checker) -> TyResult<Ty> {
        let ty = match &self.kind {
            // Infering the type of an atom is straightforward.
            ExprKind::Atom(atom) => atom.infer_ty(checker)?,
            ExprKind::Let { lhs, rhs, body } => {
                // Infer the types of both sides.
                let lhs_ty = lhs.infer_ty(checker)?;
                let rhs_ty = rhs.infer_ty(checker)?;

                // Those types have to be equal.
                checker.add_constraint(lhs_ty, rhs_ty);

                // Then the type of this expression is the type of the body.
                body.infer_ty(checker)?
            }
            ExprKind::Call { func, args } => {
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
                return_ty
            }
            ExprKind::UnaryOp { un_op, op } => {
                // Define the type the operands must have and the type that the operator returns.
                let (expected_ty, infered_ty) = match un_op {
                    // Arithmetic operators receive integers and return integers.
                    UnOp::Neg => (Ty::Base(BaseTy::Integer), Ty::Base(BaseTy::Integer)),
                    // Logic operators receive booleans and return booleans.
                    UnOp::Not => (Ty::Base(BaseTy::Bool), Ty::Base(BaseTy::Bool)),
                };

                // The operand must have the type that the operator expects.
                let ty = op.infer_ty(checker)?;
                checker.add_constraint(expected_ty, ty);

                // The type of this expression is the type that the operator returns.
                infered_ty
            }
            ExprKind::BinaryOp {
                bin_op,
                left_op,
                right_op,
            } => {
                // Define the type the operands must have and the type that the operator returns.
                let (expected_ty, infered_ty) = match bin_op {
                    // Arithmetic operators receive integers and return integers.
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem => {
                        (Ty::Base(BaseTy::Integer), Ty::Base(BaseTy::Integer))
                    }
                    // Logic operators receive booleans and return booleans.
                    BinOp::And | BinOp::Or => (Ty::Base(BaseTy::Bool), Ty::Base(BaseTy::Bool)),
                    // Equality operators receive any type and return booleans.
                    BinOp::Eq | BinOp::Neq => (checker.tcx.new_hole(), Ty::Base(BaseTy::Bool)),
                    // Comparison operators receive integers and return booleans.
                    BinOp::Lt | BinOp::Gt | BinOp::Lte | BinOp::Gte => {
                        (Ty::Base(BaseTy::Integer), Ty::Base(BaseTy::Bool))
                    }
                };

                // The operands must have the type that the operator expects.
                let left_ty = left_op.infer_ty(checker)?;
                let right_ty = right_op.infer_ty(checker)?;

                checker.add_constraint(expected_ty.clone(), left_ty);
                checker.add_constraint(expected_ty, right_ty);

                // The type of this expression is the type that the operator returns.
                infered_ty
            }
            ExprKind::Cond {
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
                do_ty
            }
            ExprKind::Tuple { fields } => {
                let fields = fields
                    .iter()
                    .map(|field| field.infer_ty(checker))
                    .collect::<TyResult<Vec<_>>>()?;

                Ty::Tuple { fields }
            }
        };

        // Store the infered type for the expression.
        checker.store_ty(self.id, ty.clone());

        Ok(ty)
    }
}
