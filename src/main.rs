use std::{env, error::Error};

use crate::scanner::Scanner;

mod scanner;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        panic!("empty args");
    }

    let filename = &args[0];

    let _file = std::fs::read(filename)?;

    Ok(())
}

fn run(source: &[u8]) -> Result<(), Box<dyn Error>> {
    let _scanner = Scanner::new(source.to_vec());
    Ok(())
}
