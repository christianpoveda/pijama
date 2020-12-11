//! Pijama's parsing module.
//!
//! This module exposes all the types required to parse a program from source code into an AST
//! representation.

mod token;

pub use token::Token;

use pijama_ast::Program;

use lalrpop_util::lalrpop_mod;
use logos::Logos;

lalrpop_mod!(parser);

/// A Lexing error.
///
/// This error type is required by LALRPOP but it is empty the lexer cannot fail, if there is an
/// invalid input, Logos will return a [Token::Error] instead.
#[derive(Debug)]
pub enum LexerError {}

/// Parse a string slice into an AST.
pub fn parse<'source>(source: &'source str) -> Program<'source> {
    // Create a new lexer and map it into an iterator that LALRPOP can handle.
    let lexer = Token::lexer(source)
        .spanned()
        .map(|(token, span)| Ok((span.start, token, span.end)));

    // FIXME: Map LALRPOP errors into something we can display.
    parser::ProgramParser::new().parse(source, lexer).unwrap()
}
