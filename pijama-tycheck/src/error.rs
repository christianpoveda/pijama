use pijama_ty::inference::Ty;

pub type TyResult<T = ()> = Result<T, TyError>;

/// A type-checking error.
///
/// Each variant here represents the reason why type-checking failed.
#[derive(Debug)]
pub enum TyError {
    /// The expected arity for a function type does not match the one found.
    ArityMismatch { expected: usize, found: usize },
    /// The expected type does not match the one found.
    TypeMismatch { expected: Ty, found: Ty },
}
