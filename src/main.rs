#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod position;
mod error;
mod values;
mod lexer;
mod parser;
mod evaluator;
use lexer::*;
use parser::*;
use evaluator::*;

use std::{env, fs};

fn main () {
    let mut args = env::args();
    args.next();
    let input_path = args.next();
    match input_path {
        None => {},
        Some(path) => {
            let res = fs::read_to_string(&path); if res.is_err() { println!("could not open {path}"); return; }
            let text = res.unwrap();

            let res = lexer::lex(&path, &text);
            if res.is_err() { println!("{}", res.err().unwrap()); return }
            let tokens = res.unwrap();
            println!("{tokens:?}");
            if tokens.len() == 0 { return }
            
            let res = parser::parse(&path, &tokens);
            if res.is_err() { println!("{}", res.err().unwrap()); return }
            let node = res.unwrap();
            println!("{node}");

            let mut context = Context::new();

            let res = evaluator::get(&node, &path, &mut context);
            if res.is_err() { println!("{}", res.err().unwrap()); return }
            let (value, ret) = res.unwrap();
            println!("{value}");
        }
    }
}