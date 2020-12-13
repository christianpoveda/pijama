use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_core as core;
use pijama_hir as hir;
use pijama_utils::index::Index;

impl Lower for hir::Local {
    type Output = core::Local;

    fn lower_with(self, _lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        // We can keep the indices between IRs because they have a one-to-one correspondence.
        Ok(core::Local::new(self.index()))
    }
}
