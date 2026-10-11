use std::{fmt, str::from_utf8};

use anyhow::{Result, bail};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
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

enum Object {
    String(String),
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    typ: TokenType,
    lexeme: String,
    line: usize,
    object: Option<Object>,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: usize, object: Option<Object>) -> Self {
        Token {
            typ,
            lexeme,
            line,
            object: None,
        }
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

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scanTokens(&mut self) -> Result<&Vec<Token>> {
        while !self.isAtEnd() {
            self.start = self.current;
            self.scanToken()?;
        }

        Ok(&self.tokens)
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current] as char;
        self.current += 1;

        c
    }

    fn scanToken(&mut self) -> anyhow::Result<char> {
        let c = self.advance();

        match c {
            '(' => self.addToken(TokenType::LeftParen, None)?,
            ')' => self.addToken(TokenType::RightParen, None)?,
            '{' => self.addToken(TokenType::LeftBrace, None)?,
            '}' => self.addToken(TokenType::RightBrace, None)?,
            ',' => self.addToken(TokenType::Comma, None)?,
            '.' => self.addToken(TokenType::Dot, None)?,
            '-' => self.addToken(TokenType::Minus, None)?,
            '+' => self.addToken(TokenType::Plus, None)?,
            ';' => self.addToken(TokenType::Semicolon, None)?,
            '*' => self.addToken(TokenType::Star, None)?,
            '!' => {
                if self.matchToken('=') {
                    self.addToken(TokenType::BangEqual, None)?;
                } else {
                    self.addToken(TokenType::Bang, None)?;
                }
            }
            '=' => {
                if self.matchToken('=') {
                    self.addToken(TokenType::EqualEqual, None)?;
                } else {
                    self.addToken(TokenType::Equal, None)?;
                }
            }
            '<' => {
                if self.matchToken('=') {
                    self.addToken(TokenType::LessEqual, None)?;
                } else {
                    self.addToken(TokenType::Less, None)?;
                }
            }
            '>' => {
                if self.matchToken('=') {
                    self.addToken(TokenType::GreaterEqual, None)?;
                } else {
                    self.addToken(TokenType::Greater, None)?;
                }
            }
            '/' => {
                if self.matchToken('/') {
                    while self.peek() != '\n' && !self.isAtEnd() {
                        self.advance();
                    }
                } else {
                    self.addToken(TokenType::Slash, None)?;
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string()?,
            _ => bail!("Unknown character: {} at line {}", c, self.line),
        };

        Ok(c)
    }

    fn string(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.isAtEnd() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.isAtEnd() {
            bail!("Unterminated string at line {}", self.line);
        }

        // closing "
        self.advance();

        let value = from_utf8(&self.source[self.start + 1..self.current - 1])?;

        self.addToken(TokenType::String, Some(Object::String(value.to_string())))
    }

    fn peek(&self) -> char {
        if self.isAtEnd() {
            return '\0';
        }

        return self.source[self.current] as char;
    }

    fn matchToken(&mut self, candidate: char) -> bool {
        if self.isAtEnd() {
            return false;
        }

        if self.source[self.current] as char != candidate {
            return false;
        }

        self.current += 1;

        true
    }

    fn addToken(&mut self, typ: TokenType, obj: Option<Object>) -> anyhow::Result<()> {
        let text = from_utf8(&self.source[self.start..self.current])?;

        self.tokens
            .push(Token::new(typ, text.to_string(), self.line, obj));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;

    #[test]
    fn scan_tokens_grouping() {
        let source = "(){}";
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        let tokens = scanner.scanTokens().unwrap();

        assert_eq!(4, tokens.len())
    }

    #[test]
    fn scan_tokens_operators() {
        let source = "!=<=>===";
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        let tokens = scanner.scanTokens().unwrap();

        assert_eq!(4, tokens.len())
    }

    #[test]
    fn scan_tokens_string() {
        let source = "\"hello world\"";
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        let tokens = scanner.scanTokens().unwrap();

        assert_eq!(1, tokens.len())
    }
}
