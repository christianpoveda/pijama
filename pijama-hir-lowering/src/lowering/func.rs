use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_hir as hir;
use pijama_mir as mir;

impl Lower for hir::Func {
    type Output = mir::Func;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        for (_, ty) in self.locals {
            // Lower the type of each local.
            let ty = lcx.lower(ty)?;
            lcx.store_local_ty(ty);
        }

        // Lower the return type.
        let return_ty = lcx.lower(self.return_ty)?;

        // Lower the body of the function.
        let body = lcx.lower(self.body)?;

        Ok(mir::Func {
            arity: self.arity,
            locals: lcx.get_local_types(),
            return_ty,
            body,
        })
    }
}
