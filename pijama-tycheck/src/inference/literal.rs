use crate::{checker::Checker, error::TyResult, inference::InferTy};

use pijama_hir::Literal;
use pijama_ty::inference::Ty;

impl InferTy for Literal {
    fn infer_ty(&self, _checker: &mut Checker) -> TyResult<Ty> {
        // The type of a literal is whatever base type it has.
        Ok(Ty::Base(self.base_ty()))
    }
}
