use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_ast as ast;
use pijama_hir as hir;

impl<'source, 'tcx> Lower<'source, 'tcx> for ast::Expr<'source> {
    type Output = hir::Expr;

    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        let expr = match self.kind {
            // Lowering an atom is straightforward.
            ast::ExprKind::Atom(atom) => hir::Expr::Atom(lcx.lower(atom)?),
            ast::ExprKind::Let {
                lhs,
                lhs_ty,
                rhs,
                body,
            } => {
                // First, lower the right-hand side of the binding (the left-hand side should not
                // be in scope yet!)
                let rhs = lcx.lower(*rhs)?;

                // Now lower the left-hand side type in order to insert it into the `locals` field
                // and get a `Local` for the left-hand side.
                let lhs_ty = lcx.lower(lhs_ty)?;
                let lhs_local = lcx.locals.insert(lhs_ty);

                // Push the left-hand side local onto the scope.
                lcx.scope.push_ident(lhs, hir::Name::Local(lhs_local));
                // Lower the body of the binding with the left-hand side in scope.
                let body = lcx.lower(*body)?;
                // Remove the left-hand side from the scope.
                lcx.scope.pop_ident();

                hir::Expr::Let {
                    lhs: lhs_local,
                    rhs: Box::new(rhs),
                    body: Box::new(body),
                }
            }
            ast::ExprKind::Call { func, args } => {
                // Lower the identifier of the called function.
                let func = lcx.lower(func)?;
                // Lower each atom sequentially.
                let args = args
                    .into_iter()
                    .map(|arg| lcx.lower(arg))
                    .collect::<LowerResult<Vec<hir::Atom>>>()?;

                hir::Expr::Call { func, args }
            }
            ast::ExprKind::UnaryOp(un_op, op) => {
                // Get the right primitive operator.
                let prim_op = match un_op.kind {
                    ast::UnOpKind::Not => hir::PrimOp::Not,
                    ast::UnOpKind::Neg => hir::PrimOp::Neg,
                };

                hir::Expr::PrimitiveOp {
                    prim_op,
                    // Lower the primitive operation with a single operand.
                    ops: vec![lcx.lower(op)?],
                }
            }
            ast::ExprKind::BinaryOp(bin_op, op1, op2) => {
                // Get the right primitive operator.
                let prim_op = match bin_op.kind {
                    ast::BinOpKind::Add => hir::PrimOp::Add,
                    ast::BinOpKind::Sub => hir::PrimOp::Sub,
                    ast::BinOpKind::Mul => hir::PrimOp::Mul,
                    ast::BinOpKind::Div => hir::PrimOp::Div,
                    ast::BinOpKind::Rem => hir::PrimOp::Rem,
                    ast::BinOpKind::And => hir::PrimOp::And,
                    ast::BinOpKind::Or => hir::PrimOp::Or,
                    ast::BinOpKind::Eq => hir::PrimOp::Eq,
                    ast::BinOpKind::Neq => hir::PrimOp::Neq,
                    ast::BinOpKind::Lt => hir::PrimOp::Lt,
                    ast::BinOpKind::Gt => hir::PrimOp::Gt,
                    ast::BinOpKind::Lte => hir::PrimOp::Lte,
                    ast::BinOpKind::Gte => hir::PrimOp::Gte,
                };

                hir::Expr::PrimitiveOp {
                    prim_op,
                    // Lower the primitive operation with two operands.
                    ops: vec![lcx.lower(op1)?, lcx.lower(op2)?],
                }
            }
            // Lowering a conditional is straightforward.
            ast::ExprKind::Cond {
                cond,
                do_branch,
                else_branch,
            } => hir::Expr::Cond {
                cond: lcx.lower(cond)?,
                do_branch: Box::new(lcx.lower(*do_branch)?),
                else_branch: Box::new(lcx.lower(*else_branch)?),
            },
        };

        Ok(expr)
    }
}
