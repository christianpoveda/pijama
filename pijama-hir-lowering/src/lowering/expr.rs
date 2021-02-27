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
            hir::ExprKind::Call { func, args } => {
                let mut binds = Vec::new();

                let mut kind = core::ExprKind::Call {
                    func: lcx.lower(func)?,
                    args: args
                        .into_iter()
                        .map(|arg| {
                            let arg = lcx.lower(arg)?;

                            let arg = match &arg.kind {
                                core::ExprKind::Atom(atom) => atom.clone(),
                                _ => {
                                    let arg_ty = lcx.get_expr_ty(arg.id).unwrap().clone();
                                    let local = lcx.store_local_ty(arg_ty);
                                    binds.push((local, arg));
                                    core::Atom::Name(core::Name::Local(local))
                                }
                            };

                            Ok(arg)
                        })
                        .collect::<LowerResult<Vec<_>>>()?,
                };

                while let Some((lhs, rhs)) = binds.pop() {
                    let rhs_ty = lcx.table.get_ty(rhs.id).unwrap().clone();

                    let id = lcx.table.store_ty(rhs_ty);

                    let body = core::Expr { id, kind };

                    kind = core::ExprKind::Let {
                        lhs,
                        rhs: Box::new(rhs),
                        body: Box::new(body),
                    }
                }

                kind
            }
            hir::ExprKind::UnaryOp { un_op, op } => {
                let un_op = match un_op {
                    hir::UnOp::Not => core::UnOp::Not,
                    hir::UnOp::Neg => core::UnOp::Neg,
                };

                let op = lcx.lower(*op)?;

                let mut bind = None;

                let op = match &op.kind {
                    core::ExprKind::Atom(atom) => atom.clone(),
                    _ => {
                        let op_ty = lcx.get_expr_ty(op.id).unwrap().clone();
                        let local = lcx.store_local_ty(op_ty);
                        bind = Some((local, op));
                        core::Atom::Name(core::Name::Local(local))
                    }
                };

                let mut kind = core::ExprKind::UnaryOp { un_op, op };

                if let Some((lhs, rhs)) = bind {
                    let rhs_ty = lcx.table.get_ty(rhs.id).unwrap().clone();

                    let id = lcx.table.store_ty(rhs_ty);

                    let body = core::Expr { id, kind };

                    kind = core::ExprKind::Let {
                        lhs,
                        rhs: Box::new(rhs),
                        body: Box::new(body),
                    }
                }

                kind
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

                let left_op = lcx.lower(*left_op)?;

                let mut left_bind = None;

                let left_op = match &left_op.kind {
                    core::ExprKind::Atom(atom) => atom.clone(),
                    _ => {
                        let op_ty = lcx.get_expr_ty(left_op.id).unwrap().clone();
                        let local = lcx.store_local_ty(op_ty);
                        left_bind = Some((local, left_op));
                        core::Atom::Name(core::Name::Local(local))
                    }
                };

                let right_op = lcx.lower(*right_op)?;

                let mut right_bind = None;

                let right_op = match &right_op.kind {
                    core::ExprKind::Atom(atom) => atom.clone(),
                    _ => {
                        let op_ty = lcx.get_expr_ty(right_op.id).unwrap().clone();
                        let local = lcx.store_local_ty(op_ty);
                        right_bind = Some((local, right_op));
                        core::Atom::Name(core::Name::Local(local))
                    }
                };

                let mut kind = core::ExprKind::BinaryOp {
                    bin_op,
                    left_op,
                    right_op,
                };

                if let Some((lhs, rhs)) = right_bind {
                    let rhs_ty = lcx.table.get_ty(rhs.id).unwrap().clone();

                    let id = lcx.table.store_ty(rhs_ty);

                    let body = core::Expr { id, kind };

                    kind = core::ExprKind::Let {
                        lhs,
                        rhs: Box::new(rhs),
                        body: Box::new(body),
                    }
                }

                if let Some((lhs, rhs)) = left_bind {
                    let rhs_ty = lcx.table.get_ty(rhs.id).unwrap().clone();

                    let id = lcx.table.store_ty(rhs_ty);

                    let body = core::Expr { id, kind };

                    kind = core::ExprKind::Let {
                        lhs,
                        rhs: Box::new(rhs),
                        body: Box::new(body),
                    }
                }

                kind
            }
            hir::ExprKind::Cond {
                cond,
                do_branch,
                else_branch,
            } => {
                let cond = lcx.lower(*cond)?;

                let mut bind = None;

                let cond = match &cond.kind {
                    core::ExprKind::Atom(atom) => atom.clone(),
                    _ => {
                        let cond_ty = lcx.get_expr_ty(cond.id).unwrap().clone();
                        let local = lcx.store_local_ty(cond_ty);
                        bind = Some((local, cond));
                        core::Atom::Name(core::Name::Local(local))
                    }
                };

                let mut kind = core::ExprKind::Cond {
                    cond,
                    do_branch: Box::new(lcx.lower(*do_branch)?),
                    else_branch: Box::new(lcx.lower(*else_branch)?),
                };

                if let Some((lhs, rhs)) = bind {
                    let rhs_ty = lcx.table.get_ty(rhs.id).unwrap().clone();

                    let id = lcx.table.store_ty(rhs_ty);

                    let body = core::Expr { id, kind };

                    kind = core::ExprKind::Let {
                        lhs,
                        rhs: Box::new(rhs),
                        body: Box::new(body),
                    }
                }

                kind
            }
        };

        Ok(core::Expr { id: self.id, kind })
    }
}
