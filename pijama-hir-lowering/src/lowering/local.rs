use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_hir as hir;
use pijama_mir as mir;
use pijama_utils::index::Index;

impl Lower for hir::Local {
    type Output = mir::Local;

    fn lower_with(self, _lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        // We can keep the indices between IRs because they have a one-to-one correspondence.
        Ok(mir::Local::new(self.index()))
    }
}
