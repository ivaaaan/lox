use std::{collections::HashMap, fmt, str::from_utf8};

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
    Number(f64),
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
    identifiers: HashMap<String, TokenType>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        let identifiers = HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]);

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            identifiers,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> anyhow::Result<char> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None)?,
            ')' => self.add_token(TokenType::RightParen, None)?,
            '{' => self.add_token(TokenType::LeftBrace, None)?,
            '}' => self.add_token(TokenType::RightBrace, None)?,
            ',' => self.add_token(TokenType::Comma, None)?,
            '.' => self.add_token(TokenType::Dot, None)?,
            '-' => self.add_token(TokenType::Minus, None)?,
            '+' => self.add_token(TokenType::Plus, None)?,
            ';' => self.add_token(TokenType::Semicolon, None)?,
            '*' => self.add_token(TokenType::Star, None)?,
            '!' => {
                if self.match_token('=') {
                    self.add_token(TokenType::BangEqual, None)?;
                } else {
                    self.add_token(TokenType::Bang, None)?;
                }
            }
            '=' => {
                if self.match_token('=') {
                    self.add_token(TokenType::EqualEqual, None)?;
                } else {
                    self.add_token(TokenType::Equal, None)?;
                }
            }
            '<' => {
                if self.match_token('=') {
                    self.add_token(TokenType::LessEqual, None)?;
                } else {
                    self.add_token(TokenType::Less, None)?;
                }
            }
            '>' => {
                if self.match_token('=') {
                    self.add_token(TokenType::GreaterEqual, None)?;
                } else {
                    self.add_token(TokenType::Greater, None)?;
                }
            }
            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)?;
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string()?,
            _ => {
                if is_digit(c) {
                    self.number()?;
                } else if is_alpha(c) {
                    self.identifier()?;
                } else {
                    bail!("Unknown character: {} at line {}", c, self.line)
                }
            }
        };

        Ok(c)
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source[self.current + 1] as char
    }

    fn match_token(&mut self, candidate: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] as char != candidate {
            return false;
        }

        self.current += 1;

        true
    }

    fn add_token(&mut self, typ: TokenType, obj: Option<Object>) -> anyhow::Result<()> {
        let text = from_utf8(&self.source[self.start..self.current])?;

        self.tokens
            .push(Token::new(typ, text.to_string(), self.line, obj));

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current] as char;
        self.current += 1;

        c
    }

    fn number(&mut self) -> Result<()> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: f64 = from_utf8(&self.source[self.start..self.current])?.parse()?;

        self.add_token(TokenType::Number, Some(Object::Number(value)))
    }

    fn identifier(&mut self) -> Result<()> {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = from_utf8(&self.source[self.start..self.current])?;
        let typ = self
            .identifiers
            .get(text)
            .cloned()
            .unwrap_or(TokenType::Identifier);

        self.add_token(typ, None)
    }

    fn string(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            bail!("Unterminated string at line {}", self.line);
        }

        // closing "
        self.advance();

        let value = from_utf8(&self.source[self.start + 1..self.current - 1])?;

        self.add_token(TokenType::String, Some(Object::String(value.to_string())))
    }
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_alpha(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;

    #[test]
    fn scan_tokens_grouping() {
        let source = "(){}";
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(4, tokens.len())
    }

    #[test]
    fn scan_tokens_operators() {
        let source = "!=<=>===";
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(4, tokens.len())
    }

    #[test]
    fn scan_tokens_string() {
        let source = "\"hello world\"";
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(1, tokens.len())
    }

    #[test]
    fn scan_tokens_number() {
        let source = "1234.5";
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(1, tokens.len())
    }

    #[test]
    fn scan_tokens_keywords() {
        let source = "or";
        let mut scanner = Scanner::new(source.as_bytes().to_vec());
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(1, tokens.len())
    }
}
