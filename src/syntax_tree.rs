use std::fmt;

use crate::scanner::{Object, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Object),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {left} {right})", operator.lexeme),
            Expr::Unary { operator, right } => write!(f, "{} {right}", operator.lexeme),
            Expr::Literal(obj) => write!(f, "{obj}"),
            Expr::Grouping(expr) => write!(f, "(group {expr})"),
        }
    }
}
