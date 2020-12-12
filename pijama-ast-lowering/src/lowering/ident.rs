use crate::{
    context::LowerContext,
    error::{LowerErrorKind, LowerResult},
    lowering::Lower,
};

use pijama_ast::Ident;
use pijama_hir::Name;

impl<'source, 'tcx> Lower<'source, 'tcx> for Ident<'source> {
    type Output = Name;

    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        // Find the identifier in the local scope.
        if let Some(name) = lcx
            .scope
            .find_ident(&self)
            // If the identifier was not in the local scope, it could be in the global scope.
            .or_else(|| lcx.global_scope.find_ident(&self))
        {
            Ok(name)
        } else {
            // Return an error if the identifier was not in scope.
            Err(LowerErrorKind::UnboundIdent(self.symbol).into_err(self.span))
        }
    }
}
