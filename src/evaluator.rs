use crate::position::*;
use crate::error::*;
use crate::values::*;
use crate::context::*;
use crate::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum R { None, Return, Break, Continue }

pub fn eval(head_value: V, head: &Node, args: Vec<V>, types: Vec<Type>, poses: Vec<&Position>, node: &Node, context: &mut Context) -> Result<(V, R), E> {
    match head_value {
        V::NativFunction(params, f) => {
            if let V::Pattern(_pattern) = params.as_ref() {
                for i in 0.._pattern.len() {
                    if &_pattern[i] != types.get(i).unwrap_or_else(|| &Type::Undefined) {
                        context.trace(&poses[i]);
                        return Err(E::ExpectedType { typ: _pattern[i].clone(), recv_typ: types[i].clone() })
                    }
                }
            } else if params.as_ref() != &V::Null {
                context.trace(&head.1);
                return Err(E::ExpectedType { typ: Type::Pattern, recv_typ: params.typ() })
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
        V::Type(typ) => {
            if args.len() == 0 { return Ok((V::Type(typ), R::None)) }
            match typ {
                Type::Function => {
                    if args[0].typ() != Type::Pattern {
                        return Err(E::ExpectedType { typ: Type::Pattern, recv_typ: args[0].typ() })
                    }
                    if args.len() >= 2 {
                        return Ok((V::Function(Box::new(args[0].clone()), Box::new(args[1].clone())), R::None))
                    }
                    return Ok((V::Function(Box::new(args[0].clone()), Box::new(V::Null)), R::None))
                }
                _ => Ok((typ.cast(&args[0]), R::None))
            }
        }
        V::Function(pattern, value) => {
            if let V::Pattern(patt_types) = pattern.as_ref() {
                for i in 0..patt_types.len() {
                    if &patt_types[i] != types.get(i).unwrap_or_else(|| &Type::Undefined) {
                        context.trace(&poses[i]);
                        return Err(E::ExpectedType { typ: patt_types[i].clone(), recv_typ: types[i].clone() })
                    }
                }
                return eval(value.as_ref().clone(), head, args, types, poses, node, context)
            }
            context.trace(&head.1);
            Err(E::ExpectedType { typ: Type::Pattern, recv_typ: pattern.typ() })
        }
        _ => {
            context.trace(&head.1);
            Err(E::HeadOperation(head_value.clone()))
        }
    }
}

pub fn get(node: &Node, context: &mut Context) -> Result<(V, R), E> {
    match &node.0 {
        N::Null => Ok((V::Null, R::None)),
        N::Wirldcard => Ok((V::Wirldcard, R::None)),
        N::Int(v) => Ok((V::Int(*v), R::None)),
        N::Float(v) => Ok((V::Float(*v), R::None)),
        N::Bool(v) => Ok((V::Bool(*v), R::None)),
        N::String(v) => Ok((V::String(v.clone()), R::None)),
        N::Type(v) => Ok((V::Type(v.clone()), R::None)),
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
        N::Pattern(nodes) => {
            let mut types: Vec<Type> = vec![];
            for n in nodes {
                let (value, _) = get(&n, context)?;
                if let V::Type(typ) = value {
                    types.push(typ);
                } else {
                    context.trace(&n.1);
                    return Err(E::ExpectedType { typ: Type::Type, recv_typ: value.typ() })
                }
            }
            Ok((V::Pattern(types), R::None))
        }
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
            let mut types: Vec<Type> = vec![];
            let mut poses: Vec<&Position> = vec![];
            for n in iter {
                let (value, _) = get(n, context)?;
                poses.push(&n.1);
                types.push(value.typ());
                args.push(value);
            }
            let (head_value, _) = get(head, context)?;
            eval(head_value, head, args, types, poses, node, context)
        }
        N::Body(nodes) => {
            for n in nodes {
                let (value, ret) = get(n, context)?;
                if ret != R::None { return Ok((value, ret)) }
            }
            Ok((V::Null, R::None))
        }
        N::Vector(nodes) => {
            let mut values: Vec<V> = vec![];
            let mut types: Vec<Type> = vec![];
            for n in nodes {
                let (value, _) = get(n, context)?;
                let typ = value.typ();
                values.push(value);
                if !types.contains(&typ) { types.push(typ) }
            }
            let mut typ = Type::Any;
            if types.len() > 0 {
                typ = types[0].clone();
                if types.len() > 1 {
                    typ = Type::Union(types);
                }
            }
            Ok((V::Vector(values, typ), R::None))
        }
    }
}