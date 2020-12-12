use crate::{checker::Checker, error::TyResult, inference::InferTy};

use pijama_hir::{Local, Name};
use pijama_ty::inference::Ty;

impl InferTy for Local {
    fn infer_ty(&self, checker: &mut Checker) -> TyResult<Ty> {
        // The type of a local is whatever it is stored in the checker.
        Ok(checker.get_name_ty(&Name::Local(*self)).unwrap().clone())
    }
}
