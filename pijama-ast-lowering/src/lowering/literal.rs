use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_ast as ast;
use pijama_hir as hir;

impl<'source, 'tcx> Lower<'source, 'tcx> for ast::Literal {
    type Output = hir::Literal;

    fn lower_with(
        self,
        _lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        // Lowering a literal is straightforward.
        Ok(match self.kind {
            ast::LiteralKind::Unit => ().into(),
            ast::LiteralKind::Bool(boolean) => boolean.into(),
            ast::LiteralKind::Integer(integer) => integer.into(),
        })
    }
}
