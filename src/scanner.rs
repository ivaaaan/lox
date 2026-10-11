use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    typ: TokenType,
    lexeme: String,
    line: u64,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: u64) -> Self {
        Token { typ, lexeme, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "type: {:?}, lexeme: {:?}, line: {}",
            self.typ, self.lexeme, self.line
        )
    }
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
        }
    }
}
