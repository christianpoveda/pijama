use crate::ident::Ident;
use crate::literal::Literal;

/// The AST representation of an atomic value.
#[derive(Debug)]
pub enum Atom<'source> {
    /// A value that can be interpreted literally.
    Literal(Literal),
    /// A value that refers to other value.
    Ident(Ident<'source>),
}
