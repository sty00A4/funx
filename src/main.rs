#![allow(unused_variables)]
#![allow(dead_code)]

mod position;
mod error;
mod values;
mod context;
mod lexer;
mod parser;
mod evaluator;
use context::*;
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
            let mut context = funx_context(&path);

            let res = lexer::lex(&text);
            if res.is_err() { println!("{}", res.err().unwrap().display(&context)); return }
            let tokens = res.unwrap();
            // println!("{tokens:?}");
            if tokens.len() == 0 { return }
            
            let res = parser::parse(&tokens, &mut context);
            if res.is_err() { println!("{}", res.err().unwrap().display(&context)); return }
            let node = res.unwrap();
            // println!("{node}");


            let res = evaluator::get(&node, &mut context);
            if res.is_err() { println!("{}", res.err().unwrap().display(&context)); return }
            let (value, ret) = res.unwrap();
            if ret != R::None { println!("{value}"); }
        }
    }
}