use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_hir as hir;
use pijama_mir as mir;
use pijama_utils::index::IndexMap;

impl Lower for hir::Program {
    type Output = mir::Program;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        let mut functions = IndexMap::new();

        for (_func_id, func) in self.functions {
            // Lower each function.
            let func = lcx.lower(func)?;
            functions.insert(func);
        }

        Ok(mir::Program { functions })
    }
}
