use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_ty::{inference, ty};

impl Lower for inference::Ty {
    type Output = ty::Ty;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        // Use the unifier to instantiate the type.
        Ok(lcx.unifier.instantiate(self))
    }
}
