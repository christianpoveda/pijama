use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_ast as ast;
use pijama_hir as hir;

impl<'source, 'tcx> Lower<'source, 'tcx> for ast::Expr<'source> {
    type Output = hir::Expr;

    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        let id = lcx.new_id();

        let kind = match self.kind {
            // Lowering an atom is straightforward.
            ast::ExprKind::Atom(atom) => hir::ExprKind::Atom(lcx.lower(atom)?),
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

                hir::ExprKind::Let {
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

                hir::ExprKind::Call { func, args }
            }
            ast::ExprKind::UnaryOp(un_op, op) => {
                let un_op = match un_op.kind {
                    ast::UnOpKind::Not => hir::UnOp::Not,
                    ast::UnOpKind::Neg => hir::UnOp::Neg,
                };

                let op = lcx.lower(op)?;

                hir::ExprKind::UnaryOp {
                    un_op,
                    op: Box::new(hir::Expr {
                        id: lcx.new_id(),
                        kind: hir::ExprKind::Atom(op),
                    }),
                }
            }
            ast::ExprKind::BinaryOp(bin_op, left_op, right_op) => {
                let bin_op = match bin_op.kind {
                    ast::BinOpKind::Add => hir::BinOp::Add,
                    ast::BinOpKind::Sub => hir::BinOp::Sub,
                    ast::BinOpKind::Mul => hir::BinOp::Mul,
                    ast::BinOpKind::Div => hir::BinOp::Div,
                    ast::BinOpKind::Rem => hir::BinOp::Rem,
                    ast::BinOpKind::And => hir::BinOp::And,
                    ast::BinOpKind::Or => hir::BinOp::Or,
                    ast::BinOpKind::Eq => hir::BinOp::Eq,
                    ast::BinOpKind::Neq => hir::BinOp::Neq,
                    ast::BinOpKind::Lt => hir::BinOp::Lt,
                    ast::BinOpKind::Gt => hir::BinOp::Gt,
                    ast::BinOpKind::Lte => hir::BinOp::Lte,
                    ast::BinOpKind::Gte => hir::BinOp::Gte,
                };

                let left_op = lcx.lower(left_op)?;
                let right_op = lcx.lower(right_op)?;

                hir::ExprKind::BinaryOp {
                    bin_op,
                    left_op: Box::new(hir::Expr {
                        id: lcx.new_id(),
                        kind: hir::ExprKind::Atom(left_op),
                    }),
                    right_op: Box::new(hir::Expr {
                        id: lcx.new_id(),
                        kind: hir::ExprKind::Atom(right_op),
                    }),
                }
            }
            // Lowering a conditional is straightforward.
            ast::ExprKind::Cond {
                cond,
                do_branch,
                else_branch,
            } => hir::ExprKind::Cond {
                cond: lcx.lower(cond)?,
                do_branch: Box::new(lcx.lower(*do_branch)?),
                else_branch: Box::new(lcx.lower(*else_branch)?),
            },
        };

        Ok(hir::Expr { id, kind })
    }
}
