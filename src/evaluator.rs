use crate::position::*;
use crate::error::*;
use crate::values::*;
use crate::context::*;
use crate::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum R { None, Return, Break, Continue }

pub fn get(node: &Node, context: &mut Context) -> Result<(V, R), E> {
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
            let (mut value, _) = get(n, context)?;
            value = Type::String.cast(&value);
            if let V::String(addr) = value {
                return Ok((V::Addr(addr), R::None))
            }
            context.trace(&node.1);
            return Err(E::ExpectedType{ typ: Type::String, recv_typ: value.typ() })
        }
        N::Arg(n) => {
            let (mut value, _) = get(n, context)?;
            value = Type::Int.cast(&value);
            if let V::Int(v) = value {
                return Ok((context.get(&v.to_string()).unwrap_or_else(||&V::Null).clone(), R::None))
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
            let mut poses: Vec<&Position> = vec![];
            for n in iter {
                let (value, _) = get(n, context)?;
                poses.push(&n.1);
                args.push(value);
            }
            let (head_value, _) = get(head, context)?;
            match head_value {
                V::NativFunction(params, f) => {
                    if args.len() < params.len() {
                        while args.len() < params.len() { args.push(V::Null); }
                    }
                    f(args, context, &node.1, &poses)
                }
                V::Bool(v) => {
                    if v && args.len() >= 1 {
                        return Ok((args[0].clone(), R::None))
                    } else if args.len() >= 2 {
                        return Ok((args[1].clone(), R::None))
                    }
                    return Ok((head_value, R::None))
                }
                V::Closure(n) => {
                    context.push();
                    context.args(&args);
                    let value_ret = get(&n, context)?;
                    context.pop();
                    return Ok(value_ret)
                }
                _ => {
                    context.trace(&node.1);
                    Err(E::HeadOperation(head_value))
                }
            }
        }
        N::Body(nodes) => {
            for n in nodes {
                let (value, ret) = get(n, context)?;
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