use crate::position::*;
use crate::error::*;
use crate::values::*;
use crate::context::*;
use crate::lexer::*;
use crate::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum R { None, Return, Break, Continue }

pub fn get(node: &Node, path: &String, context: &mut Context) -> Result<(V, R), E> {
    match &node.0 {
        N::Null => Ok((V::Null, R::None)),
        N::Wirldcard => Ok((V::Wirldcard, R::None)),
        N::Int(v) => Ok((V::Int(*v), R::None)),
        N::Float(v) => Ok((V::Float(*v), R::None)),
        N::Bool(v) => Ok((V::Bool(*v), R::None)),
        N::String(v) => Ok((V::String(v.clone()), R::None)),
        N::Addr(n) => {
            if let N::Word(addr) = &n.0 {
                return Ok((V::Addr(addr.clone()), R::None))
            }
            let (value, _) = get(n, path, context)?;
            if let V::String(addr) = value {
                return Ok((V::Addr(addr), R::None))
            }
            context.trace(&node.1);
            return Err(E::ExpectedType{ typ: Type::String, recv_typ: value.typ() })
        }
        N::Closure(n) => Ok((V::Closure(n.as_ref().clone()), R::None)),
        N::Word(word) => {
            let v = context.get(word);
            if let Some(value) = v {
                return Ok((value.clone(), R::None))
            }
            return Ok((V::Null, R::None))
        }
        N::Eval(nodes) => {
            if nodes.len() == 0 { return Ok((V::Null, R::None)) }
            let mut iter = nodes.iter();
            let head = iter.next().unwrap();
            let mut args: Vec<V> = vec![];
            for n in iter {
                let (value, _) = get(n, path, context)?;
                args.push(value);
            }
            let (head_value, _) = get(head, path, context)?;
            match head_value {
                V::NativFunction(params, f) => {
                    if args.len() < params.len() {
                        while args.len() < params.len() { args.push(V::Null); }
                    }
                    f(args, context)
                }
                _ => {
                    context.trace(&node.1);
                    Err(E::HeadOperation(head_value))
                }
            }
        }
        N::Body(nodes) => {
            for n in nodes {
                let (value, ret) = get(n, path, context)?;
                if ret != R::None { return Ok((value, ret)) }
            }
            Ok((V::Null, R::None))
        }
        N::Pattern(nodes) => {
            Ok((V::Null, R::None))
        }
        N::Vector(nodes) => {
            Ok((V::Null, R::None))
        }
    }
}