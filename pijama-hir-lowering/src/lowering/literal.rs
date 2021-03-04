use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_core as core;
use pijama_hir as hir;
use pijama_ty::base::BaseTy;

impl Lower for hir::Literal {
    type Output = core::Literal;

    fn lower_with(self, _lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        // FIXME: maybe these two types should be the same.
        let literal = match self.base_ty() {
            BaseTy::Bool => (self.bits() != 0).into(),
            BaseTy::Int => self.bits().into(),
        };

        Ok(literal)
    }
}
