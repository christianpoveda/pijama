use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_hir as hir;
use pijama_mir as mir;

impl Lower for hir::Expr {
    type Output = mir::Expr;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        let mut binds = Vec::new();

        let mut kind = match self.kind {
            hir::ExprKind::Atom(atom) => mir::ExprKind::Atom(lcx.lower(atom)?),
            hir::ExprKind::Let { lhs, rhs, body } => mir::ExprKind::Let {
                lhs: lcx.lower(lhs)?,
                rhs: lcx.lower(rhs)?,
                body: lcx.lower(body)?,
            },
            hir::ExprKind::Call { func, args } => {
                // FIXME: allow arbitrary funcs
                let func = lcx.lower(func)?;

                let args = args
                    .into_iter()
                    .map(|expr| lower_into_atom(expr, lcx, &mut binds))
                    .collect::<LowerResult<Vec<_>>>()?;

                mir::ExprKind::Call { func, args }
            }
            hir::ExprKind::UnaryOp { un_op, op } => {
                let un_op = match un_op {
                    hir::UnOp::Not => mir::UnOp::Not,
                    hir::UnOp::Neg => mir::UnOp::Neg,
                };

                let op = lower_into_atom(*op, lcx, &mut binds)?;

                mir::ExprKind::UnaryOp { un_op, op }
            }
            hir::ExprKind::BinaryOp {
                bin_op,
                left_op,
                right_op,
            } => {
                let bin_op = match bin_op {
                    hir::BinOp::Add => mir::BinOp::Add,
                    hir::BinOp::Sub => mir::BinOp::Sub,
                    hir::BinOp::Mul => mir::BinOp::Mul,
                    hir::BinOp::Div => mir::BinOp::Div,
                    hir::BinOp::Rem => mir::BinOp::Rem,
                    hir::BinOp::And => mir::BinOp::And,
                    hir::BinOp::Or => mir::BinOp::Or,
                    hir::BinOp::Eq => mir::BinOp::Eq,
                    hir::BinOp::Neq => mir::BinOp::Neq,
                    hir::BinOp::Lt => mir::BinOp::Lt,
                    hir::BinOp::Gt => mir::BinOp::Gt,
                    hir::BinOp::Lte => mir::BinOp::Lte,
                    hir::BinOp::Gte => mir::BinOp::Gte,
                };

                let left_op = lower_into_atom(*left_op, lcx, &mut binds)?;
                let right_op = lower_into_atom(*right_op, lcx, &mut binds)?;

                mir::ExprKind::BinaryOp {
                    bin_op,
                    left_op,
                    right_op,
                }
            }
            hir::ExprKind::Cond {
                cond,
                do_branch,
                else_branch,
            } => mir::ExprKind::Cond {
                cond: lower_into_atom(*cond, lcx, &mut binds)?,
                do_branch: lcx.lower(do_branch)?,
                else_branch: lcx.lower(else_branch)?,
            },
            hir::ExprKind::Tuple { fields } => {
                let fields = fields
                    .into_iter()
                    .map(|expr| lower_into_atom(expr, lcx, &mut binds))
                    .collect::<LowerResult<Vec<_>>>()?;

                mir::ExprKind::Tuple { fields }
            }
        };

        let ty = lcx.get_expr_ty(self.id).unwrap().clone();

        for (lhs, rhs) in binds.into_iter().rev() {
            let body = mir::Expr {
                id: lcx.table.store_ty(ty.clone()),
                kind,
            };

            kind = mir::ExprKind::Let {
                lhs,
                rhs: Box::new(rhs),
                body: Box::new(body),
            };
        }

        Ok(mir::Expr { id: self.id, kind })
    }
}

fn lower_into_atom(
    expr: hir::Expr,
    lcx: &mut LowerContext,
    binds: &mut Vec<(mir::Local, mir::Expr)>,
) -> LowerResult<mir::Atom> {
    match lcx.lower(expr)? {
        mir::Expr {
            kind: mir::ExprKind::Atom(atom),
            ..
        } => Ok(atom),
        expr => {
            let expr_ty = lcx.get_expr_ty(expr.id).unwrap().clone();
            let local = lcx.store_local_ty(expr_ty);

            binds.push((local, expr));

            Ok(mir::Atom::Name(mir::Name::Local(local)))
        }
    }
}
