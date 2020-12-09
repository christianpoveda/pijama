//! Base types.

/// A base type.
#[derive(Debug, Copy, Clone)]
pub enum BaseTy {
    /// The unit type.
    Unit,
    /// The boolean type.
    Bool,
    /// The signed integer type.
    Integer,
}
