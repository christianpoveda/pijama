use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_hir as hir;
use pijama_mir as mir;
use pijama_utils::index::Index;

impl Lower for hir::Name {
    type Output = mir::Name;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        let name = match self {
            hir::Name::Local(local) => mir::Name::Local(lcx.lower(local)?),
            // We can keep the indices between IRs because they have a one-to-one correspondence.
            hir::Name::FuncPtr(func_id) => mir::Name::FuncPtr(mir::FuncId::new(func_id.index())),
        };

        Ok(name)
    }
}
