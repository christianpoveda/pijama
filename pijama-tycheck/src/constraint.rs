use pijama_ty::inference::Ty;

pub struct Constraint {
    /// The left-hand side of the constraint. Usually this is the expected type of an expression.
    pub(crate) lhs: Ty,
    /// The right-hand side of the constraint. Usually this is the infered type for an expression.
    pub(crate) rhs: Ty,
}

impl Constraint {
    /// Create a new constraint.
    pub(crate) fn new(lhs: Ty, rhs: Ty) -> Self {
        Self { lhs, rhs }
    }
}
