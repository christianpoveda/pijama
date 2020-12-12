use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_ast as ast;
use pijama_hir as hir;
use pijama_utils::index::IndexMap;

impl<'source, 'tcx> Lower<'source, 'tcx> for ast::FuncDef<'source> {
    type Output = hir::Func;

    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        // Compute the arity of the function.
        let arity = self.params.len();

        for (param_ident, param_ty) in self.params {
            // Lower the type of each parameter and get a local for the parameter by inserting it
            // into the `locals` field.
            let param_ty = lcx.lower(param_ty)?;
            let param_local = lcx.locals.insert(param_ty);
            // Push the local into scope.
            lcx.scope
                .push_ident(param_ident, hir::Name::Local(param_local));
        }

        // Lower the body with all the parameters in scope.
        let body = lcx.lower(self.body)?;

        // Remove all the parameters from the scope.
        for _ in 0..arity {
            lcx.scope.pop_ident();
        }

        // Take the `locals` field and replace it with an empty map.
        let locals = std::mem::replace(&mut lcx.locals, IndexMap::new());

        // Lower the return type.
        let return_ty = lcx.lower(self.return_ty)?;

        Ok(hir::Func {
            arity,
            locals,
            return_ty,
            body,
        })
    }
}
