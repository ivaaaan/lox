use anyhow::{Result, bail};

use crate::{
    scanner::{Object, Token, TokenType},
    syntax_tree::Expr,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.match_type(&[TokenType::Bang, TokenType::BangEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.match_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.match_type(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        if self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        let types = vec![TokenType::Bang, TokenType::Minus];

        if self.match_type(&types) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.match_type(&[TokenType::False]) {
            return Ok(Expr::Literal(Object::Boolean(false)));
        }

        if self.match_type(&[TokenType::True]) {
            return Ok(Expr::Literal(Object::Boolean(true)));
        }

        if self.match_type(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Object::Nil));
        }

        if self.match_type(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().object.unwrap()));
        }

        if self.match_type(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "expect ')' after expression.".to_string(),
            )?;

            return Ok(Expr::Grouping(Box::new(expr)));
        }

        bail!("Expect expression.");
    }

    fn consume(&mut self, typ: TokenType, message: String) -> Result<Token> {
        if self.check(&typ) {
            return Ok(self.advance());
        }

        bail!(message)
    }

    fn match_type(&mut self, types: &[TokenType]) -> bool {
        for typ in types {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, typ: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().typ.eq(typ)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().typ.eq(&TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().typ.eq(&TokenType::Semicolon) {
                return;
            }

            if matches!(
                self.peek().typ,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
        }

        self.advance();
    }
}
