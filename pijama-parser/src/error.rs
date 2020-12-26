use crate::token::Token;

use pijama_utils::{span::Span, spanned_type};

use lalrpop_util::ParseError as LalrpopError;

pub type ParseResult<'source, T> = Result<T, ParseError<'source>>;

spanned_type!(pub ParseError<'source>, ParseErrorKind);

/// A Parsing error.
///
/// This type represents a reason why parsing failed.
#[derive(Debug)]
pub enum ParseErrorKind<'source> {
    /// The source input ended unexpectedly.
    UnexpectedEOF { expected: Vec<String> },
    /// The source input has a valid but unexpected token.
    UnexpectedToken {
        expected: Vec<String>,
        found: Token<'source>,
    },
    /// The source input has an invalid token.
    InvalidToken,
}

impl<'source> From<LalrpopError<usize, Token<'source>, LexerError>> for ParseError<'source> {
    fn from(error: LalrpopError<usize, Token<'source>, LexerError>) -> Self {
        match error {
            LalrpopError::InvalidToken { location } => ParseError {
                kind: ParseErrorKind::InvalidToken,
                span: Span::new(location, location),
            },
            LalrpopError::UnrecognizedEOF { location, expected } => ParseError {
                kind: ParseErrorKind::UnexpectedEOF { expected },
                span: Span::new(location, location),
            },
            LalrpopError::UnrecognizedToken {
                token: (start, found, end),
                expected,
            } => ParseError {
                kind: ParseErrorKind::UnexpectedToken { expected, found },
                span: Span::new(start, end),
            },
            LalrpopError::ExtraToken {
                token: (start, found, end),
            } => ParseError {
                kind: ParseErrorKind::UnexpectedToken {
                    expected: vec![],
                    found,
                },
                span: Span::new(start, end),
            },
            // Lexing errors are impossible here.
            LalrpopError::User { .. } => unreachable!(),
        }
    }
}

/// A Lexing error.
///
/// This error type is required by LALRPOP but it is empty the lexer cannot fail, if there is an
/// invalid input, Logos will return a [Token::Error] instead.
#[derive(Debug)]
pub enum LexerError {}
