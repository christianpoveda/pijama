use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_core as core;
use pijama_hir as hir;

impl Lower for hir::Expr {
    type Output = core::Expr;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        // Lowering expressions is straightforward because they are the same.
        let expr = match self {
            hir::Expr::Atom(atom) => core::Expr::Atom(lcx.lower(atom)?),
            hir::Expr::Let { lhs, rhs, body } => core::Expr::Let {
                lhs: lcx.lower(lhs)?,
                rhs: Box::new(lcx.lower(*rhs)?),
                body: Box::new(lcx.lower(*body)?),
            },
            hir::Expr::Call { func, args } => core::Expr::Call {
                func: lcx.lower(func)?,
                args: args
                    .into_iter()
                    .map(|arg| lcx.lower(arg))
                    .collect::<LowerResult<Vec<_>>>()?,
            },
            hir::Expr::UnaryOp { un_op, op } => {
                let un_op = match un_op {
                    hir::UnOp::Not => core::UnOp::Not,
                    hir::UnOp::Neg => core::UnOp::Neg,
                };

                core::Expr::UnaryOp {
                    un_op,
                    op: lcx.lower(op)?,
                }
            }
            hir::Expr::BinaryOp {
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

                core::Expr::BinaryOp {
                    bin_op,
                    left_op: lcx.lower(left_op)?,
                    right_op: lcx.lower(right_op)?,
                }
            }
            hir::Expr::Cond {
                cond,
                do_branch,
                else_branch,
            } => core::Expr::Cond {
                cond: lcx.lower(cond)?,
                do_branch: Box::new(lcx.lower(*do_branch)?),
                else_branch: Box::new(lcx.lower(*else_branch)?),
            },
        };

        Ok(expr)
    }
}
