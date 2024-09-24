use std::fmt;
use std::fmt::Debug;
use crate::lexer::span::TextSpan;

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut struc = f.debug_struct("Token");

        struc.field("kind", &self.kind).field("span", &self.span);

        struc.finish()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Separators
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,

    // Literals
    Identifier(String),
    String(String),
    Rational(f64),
    Integer(i64),
    Boolean(bool),

    // Keywords
    Fn,
    Let,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    Use,
    Export,

    True,
    False,
    Null,

    // Operators
    Plus,               // +
    Minus,              // -
    Asterisk,           // *
    Slash,              // /
    Equals,             // =
    Ampersand,          // &
    Pipe,               // |
    Caret,              // ^
    DoubleAsterisk,     // **
    Percent,            // %
    Tilde,              // ~
    GreaterThan,        // >
    LessThan,           // <
    GreaterThanEquals,  // >=
    LessThanEquals,     // <=
    EqualsEquals,       // ==
    BangEquals,         // !=
    Bang,                // !
    And,                // &&
    Or,                 // ||
    Increment,          // ++
    Decrement,          // --
    MinusEquals,        // -=
    PlusEquals,         // +=

    EOF,
    Whitespace,
    Bad,
    Comment,
}

