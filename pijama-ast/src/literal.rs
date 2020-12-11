use pijama_utils::spanned_type;

spanned_type!(pub Literal, LiteralKind);

/// The AST representation of a literal value.
#[derive(Debug)]
pub enum LiteralKind {
    /// A literal of type `Unit`.
    Unit,
    /// A literal of type `Bool`.
    Bool(bool),
    /// A literal of type `Int`.
    Integer(i64),
}
