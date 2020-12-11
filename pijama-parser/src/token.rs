use logos::Logos;

/// A token for Pijama's syntax.
#[derive(Logos, Debug, Clone)]
pub enum Token<'source> {
    /// A 64-bit, signed integer.
    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
    /// An identifier for a value or type.
    ///
    /// Pijama follows the same rules for correctness of identifiers as Rust.
    #[regex("[a-zA-Z][a-zA-Z0-9_]*|_[a-zA-Z0-9_]+")]
    Ident(&'source str),
    /// The `true` token.
    #[token("true")]
    True,
    /// The `false` token.
    #[token("false")]
    False,
    /// The `unit` token.
    #[token("unit")]
    Unit,
    /// The `let` token.
    #[token("let")]
    Let,
    /// The `in` token.
    #[token("in")]
    In,
    /// The `fn` token.
    #[token("fn")]
    Fn,
    /// The `if` token.
    #[token("if")]
    If,
    /// The `do` token.
    #[token("do")]
    Do,
    /// The `else` token.
    #[token("else")]
    Else,
    /// The `end` token.
    #[token("end")]
    End,
    /// The `+` token.
    #[token("+")]
    Add,
    /// The `-` token.
    #[token("-")]
    Sub,
    /// The `*` token.
    #[token("*")]
    Mul,
    /// The `/` token.
    #[token("/")]
    Div,
    /// The `%` token.
    #[token("%")]
    Rem,
    /// The `&&` token.
    #[token("&&")]
    And,
    /// The `||` token.
    #[token("||")]
    Or,
    /// The `!` token.
    #[token("!")]
    Not,
    /// The `==` token.
    #[token("==")]
    Eq,
    /// The `!=` token.
    #[token("!=")]
    Neq,
    /// The `>` token.
    #[token(">")]
    Gt,
    /// The `<` token.
    #[token("<")]
    Lt,
    /// The `>=` token.
    #[token(">=")]
    Gte,
    /// The `<=` token.
    #[token("<=")]
    Lte,
    /// The `=` token.
    #[token("=")]
    Assign,
    /// The `:` token.
    #[token(":")]
    Colon,
    /// The `,` token.
    #[token(",")]
    Comma,
    /// The `->` token.
    #[token("->")]
    Arrow,
    /// The `(` token.
    #[token("(")]
    OpenParen,
    /// The `)` token.
    #[token(")")]
    CloseParen,
    /// A placeholder token for errors.
    ///
    /// This variant is required by Logos and it is the only way to handle lexing errors.
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
