//! Base types.

use pijama_utils::show::Show;

/// A base type.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BaseTy {
    /// The boolean type.
    Bool,
    /// The signed integer type.
    Int,
}

impl<Ctx> Show<Ctx> for BaseTy {
    fn show(&self, _ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "Bool"),
            Self::Int => write!(f, "Int"),
        }
    }
}
