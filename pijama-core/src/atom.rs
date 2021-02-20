use crate::{literal::Literal, name::Name};

/// An atomic value.
///
/// Atoms represent values that do not need to be computed.
#[derive(Debug, Clone)]
pub enum Atom {
    /// A value that can be interpreted literally.
    Literal(Literal),
    /// A value that refers to other value.
    Name(Name),
}
