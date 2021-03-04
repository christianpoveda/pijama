use crate::{
    context::LowerContext,
    error::{LowerErrorKind, LowerResult},
    lowering::Lower,
};

use pijama_ast as ast;
use pijama_ty::{base::BaseTy, inference::Ty};

impl<'source, 'tcx> Lower<'source, 'tcx> for ast::Ty<'source> {
    type Output = Ty;

    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        match self.kind {
            ast::TyKind::Base(ident) => match ident.symbol {
                // Check that base types have the right symbol.
                "Bool" => Ok(Ty::Base(BaseTy::Bool)),
                "Int" => Ok(Ty::Base(BaseTy::Int)),
                // Return an error if the symbol is not right.
                symbol => Err(LowerErrorKind::UnboundIdent(symbol).into_err(ident.span)),
            },
            // Lower function types recursively.
            ast::TyKind::Func {
                params_ty,
                return_ty,
            } => Ok(Ty::Func {
                params_ty: params_ty
                    .into_iter()
                    .map(|ty| lcx.lower(ty))
                    .collect::<LowerResult<Vec<Ty>>>()?,
                return_ty: Box::new(lcx.lower(*return_ty)?),
            }),
            ast::TyKind::Tuple { fields } => Ok(Ty::Tuple {
                fields: fields
                    .into_iter()
                    .map(|ty| lcx.lower(ty))
                    .collect::<LowerResult<Vec<Ty>>>()?,
            }),
        }
    }
}

impl<'source, 'tcx> Lower<'source, 'tcx> for Option<ast::Ty<'source>> {
    type Output = Ty;

    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        if let Some(ty) = self {
            lcx.lower(ty)
        } else {
            Ok(lcx.tcx.new_hole())
        }
    }
}
