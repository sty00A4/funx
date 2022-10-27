#![allow(unused_variables)]
#![allow(dead_code)]

mod position;
mod error;
mod values;
mod context;
mod lexer;
mod parser;
mod evaluator;
use error::*;
use values::*;
use context::*;
use evaluator::*;

use std::{env, fs};

pub fn run(path: &String, text: &String, context: &mut Context) -> Result<(V, R), E> {
    let tokens = lexer::lex(&text)?;
    // println!("{tokens:?}");
    if tokens.len() == 0 { return Ok((V::Null, R::None)) }
    
    let node = parser::parse(&tokens, context)?;
    // println!("{node}");

    evaluator::get(&node, context)
}
pub fn runfile(path: &String, context: &mut Context) -> Result<(V, R), E> {
    let res = fs::read_to_string(path);
    if res.is_err() { return Err(E::FileNotFound(path.clone())) }
    let text = res.unwrap();

    run(path, &text, context)
}

fn main () {
    let mut args = env::args();
    args.next();
    let input_path = args.next();
    match input_path {
        None => {},
        Some(path) => {
            let mut context = funx_context(&path);
            let res = runfile(&path, &mut context);
            if res.is_err() { println!("{}", res.err().unwrap().display(&context)); return }
        }
    }
}