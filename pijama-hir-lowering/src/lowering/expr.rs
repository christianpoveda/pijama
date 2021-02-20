use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_core as core;
use pijama_hir as hir;

impl Lower for hir::Expr {
    type Output = core::Expr;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        // Lowering expressions is straightforward because they are the same.
        let kind = match self.kind {
            hir::ExprKind::Atom(atom) => core::ExprKind::Atom(lcx.lower(atom)?),
            hir::ExprKind::Let { lhs, rhs, body } => core::ExprKind::Let {
                lhs: lcx.lower(lhs)?,
                rhs: Box::new(lcx.lower(*rhs)?),
                body: Box::new(lcx.lower(*body)?),
            },
            hir::ExprKind::Call { func, args } => core::ExprKind::Call {
                func: lcx.lower(func)?,
                args: args
                    .into_iter()
                    .map(|arg| lcx.lower(arg))
                    .collect::<LowerResult<Vec<_>>>()?,
            },
            hir::ExprKind::UnaryOp { un_op, op } => {
                let un_op = match un_op {
                    hir::UnOp::Not => core::UnOp::Not,
                    hir::UnOp::Neg => core::UnOp::Neg,
                };

                core::ExprKind::UnaryOp {
                    un_op,
                    op: lcx.lower(op)?,
                }
            }
            hir::ExprKind::BinaryOp {
                bin_op,
                left_op,
                right_op,
            } => {
                let bin_op = match bin_op {
                    hir::BinOp::Add => core::BinOp::Add,
                    hir::BinOp::Sub => core::BinOp::Sub,
                    hir::BinOp::Mul => core::BinOp::Mul,
                    hir::BinOp::Div => core::BinOp::Div,
                    hir::BinOp::Rem => core::BinOp::Rem,
                    hir::BinOp::And => core::BinOp::And,
                    hir::BinOp::Or => core::BinOp::Or,
                    hir::BinOp::Eq => core::BinOp::Eq,
                    hir::BinOp::Neq => core::BinOp::Neq,
                    hir::BinOp::Lt => core::BinOp::Lt,
                    hir::BinOp::Gt => core::BinOp::Gt,
                    hir::BinOp::Lte => core::BinOp::Lte,
                    hir::BinOp::Gte => core::BinOp::Gte,
                };

                core::ExprKind::BinaryOp {
                    bin_op,
                    left_op: lcx.lower(left_op)?,
                    right_op: lcx.lower(right_op)?,
                }
            }
            hir::ExprKind::Cond {
                cond,
                do_branch,
                else_branch,
            } => core::ExprKind::Cond {
                cond: lcx.lower(cond)?,
                do_branch: Box::new(lcx.lower(*do_branch)?),
                else_branch: Box::new(lcx.lower(*else_branch)?),
            },
        };

        Ok(core::Expr { id: self.id, kind })
    }
}
