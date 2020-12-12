mod atom;
mod expr;
mod literal;
mod local;

use crate::{checker::Checker, error::TyResult};

use pijama_ty::inference::Ty;

/// A trait that every term whose type can be infered by the [Checker] must implement.
pub(crate) trait InferTy {
    /// Infer the type of this term.
    fn infer_ty(&self, checker: &mut Checker) -> TyResult<Ty>;
}
