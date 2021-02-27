//! Concrete types.
//!
//! This is the type representation used after type inference.
use crate::base::BaseTy;

/// A concrete type.
#[derive(Debug, Clone)]
pub enum Ty {
    /// A base type.
    Base(BaseTy),
    /// A function type.
    Func {
        /// The type of each parameter.
        params_ty: Vec<Self>,
        /// The return type.
        return_ty: Box<Self>,
    },
    /// A tuple type.
    Tuple { fields: Vec<Self> },
}
