mod base_ops;
mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

use std::{env, fs};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::parser::parse_simlang;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let contents = fs::read_to_string(&args[1]).expect("cannot read file");
        parse_simlang(&contents)
            .expect("failed to parse file!")
            .into_iter()
            .for_each(|parsed| {
                println!("{}", base_ops::compile(parsed).as_str())
            });
    } else {
        let mut reader = Editor::<()>::new().unwrap();
        loop {
            let line = reader.readline(">> ");
            match line {
                Ok(contents) => {
                    for parsed in parse_simlang(&contents).unwrap() {
                        println!("{}", base_ops::compile(parsed).as_str());
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL+C!");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL+D!");
                    break;
                }
                Err(err) => {
                    println!("Unexpected error: {:?}", err);
                    break;
                }
            }
        }
    }
}
