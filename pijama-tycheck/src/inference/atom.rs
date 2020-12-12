use crate::{checker::Checker, error::TyResult, inference::InferTy};

use pijama_hir::Atom;
use pijama_ty::inference::Ty;

impl InferTy for Atom {
    fn infer_ty(&self, checker: &mut Checker) -> TyResult<Ty> {
        // Infering the type of an atom is straightforward.
        match self {
            Atom::Literal(literal) => literal.infer_ty(checker),
            Atom::Name(name) => Ok(checker.get_name_ty(name).unwrap().clone()),
        }
    }
}
