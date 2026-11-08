use std::{env, error::Error};

use crate::{
    parser::Parser,
    scanner::{Scanner, Token, TokenType},
    syntax_tree::Expr,
};

mod parser;
mod scanner;
mod syntax_tree;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        panic!("empty args");
    }

    let filename = &args[1];

    let file = std::fs::read(filename)?;

    // let expr = Expr::Binary {
    //     left: Box::new(Expr::Literal(scanner::Object::String("hello".to_string()))),
    //     operator: Token::new(TokenType::Plus, "+".to_string(), 1, None),
    //     right: Box::new(Expr::Literal(scanner::Object::String("world".to_string()))),
    // };

    // println!("{}", expr);

    run(&file)
}

fn run(source: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(source.to_vec());
    let tokens = scanner.scan_tokens()?;

    dbg!(tokens);

    let mut parser = Parser::new(tokens);

    let expr = parser.parse();

    if let Some(v) = expr {
        println!("{}", v);
    }

    Ok(())
}
