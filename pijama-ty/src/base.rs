//! Base types.

/// A base type.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BaseTy {
    /// The boolean type.
    Bool,
    /// The signed integer type.
    Integer,
}
