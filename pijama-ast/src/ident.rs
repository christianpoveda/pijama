use pijama_utils::span::Span;

use std::fmt;

// The identifier for an item in the AST.
#[derive(Clone)]
pub struct Ident<'source> {
    /// The string representation of this identifier.
    pub symbol: &'source str,
    /// The span of the identifier.
    pub span: Span,
}

impl<'source> fmt::Debug for Ident<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}@{:?}\"", self.symbol, self.span)
    }
}
