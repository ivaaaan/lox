use std::{env, error::Error};

use crate::{
    scanner::{Scanner, Token, TokenType},
    syntax_tree::Expr,
};

mod scanner;
mod syntax_tree;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        panic!("empty args");
    }

    // let filename = &args[0];
    //
    // let _file = std::fs::read(filename)?;
    //
    //
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal(scanner::Object::String("hello".to_string()))),
        operator: Token::new(TokenType::Plus, "+".to_string(), 1, None),
        right: Box::new(Expr::Literal(scanner::Object::String("world".to_string()))),
    };

    println!("{}", expr);

    Ok(())
}

fn run(source: &[u8]) -> Result<(), Box<dyn Error>> {
    let _scanner = Scanner::new(source.to_vec());
    Ok(())
}
