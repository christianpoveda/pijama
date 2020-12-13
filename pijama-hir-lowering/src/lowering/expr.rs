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
            hir::Expr::PrimitiveOp { prim_op, ops } => {
                let prim_op = match prim_op {
                    hir::PrimOp::Not => core::PrimOp::Not,
                    hir::PrimOp::Neg => core::PrimOp::Neg,
                    hir::PrimOp::Add => core::PrimOp::Add,
                    hir::PrimOp::Sub => core::PrimOp::Sub,
                    hir::PrimOp::Mul => core::PrimOp::Mul,
                    hir::PrimOp::Div => core::PrimOp::Div,
                    hir::PrimOp::Rem => core::PrimOp::Rem,
                    hir::PrimOp::And => core::PrimOp::And,
                    hir::PrimOp::Or => core::PrimOp::Or,
                    hir::PrimOp::Eq => core::PrimOp::Eq,
                    hir::PrimOp::Neq => core::PrimOp::Neq,
                    hir::PrimOp::Lt => core::PrimOp::Lt,
                    hir::PrimOp::Gt => core::PrimOp::Gt,
                    hir::PrimOp::Lte => core::PrimOp::Lte,
                    hir::PrimOp::Gte => core::PrimOp::Gte,
                };

                core::Expr::PrimitiveOp {
                    prim_op,
                    ops: ops
                        .into_iter()
                        .map(|arg| lcx.lower(arg))
                        .collect::<LowerResult<Vec<_>>>()?,
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
