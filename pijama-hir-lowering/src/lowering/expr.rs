use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_core as core;
use pijama_hir as hir;

impl Lower for hir::Expr {
    type Output = core::Expr;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        let mut binds = Vec::new();

        let mut kind = match self.kind {
            hir::ExprKind::Atom(atom) => core::ExprKind::Atom(lcx.lower(atom)?),
            hir::ExprKind::Let { lhs, rhs, body } => core::ExprKind::Let {
                lhs: lcx.lower(lhs)?,
                rhs: Box::new(lcx.lower(*rhs)?),
                body: Box::new(lcx.lower(*body)?),
            },
            hir::ExprKind::Call { func, args } => {
                // FIXME: allow arbitrary funcs
                let func = lcx.lower(func)?;

                let args = args
                    .into_iter()
                    .map(|expr| lower_into_atom(expr, lcx, &mut binds))
                    .collect::<LowerResult<Vec<_>>>()?;

                core::ExprKind::Call { func, args }
            }
            hir::ExprKind::UnaryOp { un_op, op } => {
                let un_op = match un_op {
                    hir::UnOp::Not => core::UnOp::Not,
                    hir::UnOp::Neg => core::UnOp::Neg,
                };

                let op = lower_into_atom(*op, lcx, &mut binds)?;

                core::ExprKind::UnaryOp { un_op, op }
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

                let left_op = lower_into_atom(*left_op, lcx, &mut binds)?;
                let right_op = lower_into_atom(*right_op, lcx, &mut binds)?;

                core::ExprKind::BinaryOp {
                    bin_op,
                    left_op,
                    right_op,
                }
            }
            hir::ExprKind::Cond {
                cond,
                do_branch,
                else_branch,
            } => {
                let cond = lower_into_atom(*cond, lcx, &mut binds)?;
                let do_branch = lcx.lower(*do_branch)?;
                let else_branch = lcx.lower(*else_branch)?;

                core::ExprKind::Cond {
                    cond,
                    do_branch: Box::new(do_branch),
                    else_branch: Box::new(else_branch),
                }
            }
            hir::ExprKind::Tuple { fields } => {
                let fields = fields
                    .into_iter()
                    .map(|expr| lower_into_atom(expr, lcx, &mut binds))
                    .collect::<LowerResult<Vec<_>>>()?;

                core::ExprKind::Tuple { fields }
            }
        };

        let ty = lcx.get_expr_ty(self.id).unwrap().clone();

        for (lhs, rhs) in binds.into_iter().rev() {
            let body = core::Expr {
                id: lcx.table.store_ty(ty.clone()),
                kind,
            };

            kind = core::ExprKind::Let {
                lhs,
                rhs: Box::new(rhs),
                body: Box::new(body),
            };
        }

        Ok(core::Expr { id: self.id, kind })
    }
}

fn lower_into_atom(
    expr: hir::Expr,
    lcx: &mut LowerContext,
    binds: &mut Vec<(core::Local, core::Expr)>,
) -> LowerResult<core::Atom> {
    match lcx.lower(expr)? {
        core::Expr {
            kind: core::ExprKind::Atom(atom),
            ..
        } => Ok(atom),
        expr => {
            let expr_ty = lcx.get_expr_ty(expr.id).unwrap().clone();
            let local = lcx.store_local_ty(expr_ty);

            binds.push((local, expr));

            Ok(core::Atom::Name(core::Name::Local(local)))
        }
    }
}
