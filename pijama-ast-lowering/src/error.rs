use pijama_utils::{span::Span, spanned_type};

pub type LowerResult<'source, T> = Result<T, LowerError<'source>>;

spanned_type!(pub LowerError<'source>, LowerErrorKind);

/// A lowering error.
///
/// Each variant here represents the reason why it was not possible to lower the AST
/// representation.
#[derive(Debug)]
pub enum LowerErrorKind<'source> {
    /// An identifier was used without being bound.
    UnboundIdent(&'source str),
    /// The current program does not have a `main` function.
    MainNotFound,
}

impl<'source> LowerErrorKind<'source> {
    /// Consume the current kind to return an error.
    pub(crate) fn into_err(self, span: Span) -> LowerError<'source> {
        LowerError { kind: self, span }
    }
}
