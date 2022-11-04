use crate::position::*;
use crate::error::*;
use crate::runfile;
use crate::values::*;
use crate::evaluator::*;

#[derive(Debug, Clone)]
pub struct Scope {
    vars: Vec<(String, V)>,
    args: Vec<V>
}
impl Scope {
    pub fn new() -> Self { Self { vars: vec![], args: vec![] } }
    pub fn var(&mut self, word: &String, value: &V) -> Result<(), ()> {
        for (var, _) in self.vars.iter() {
            if word == var { return Err(()) }
        }
        self.vars.push((word.clone(), value.clone()));
        Ok(())
    }
    pub fn set(&mut self, word: &String, value: &V) -> Result<(), ()> {
        for (var, v) in self.vars.iter_mut() {
            if word == var { *v = value.clone(); return Ok(()) }
        }
        Err(())
    }
    pub fn get(&self, word: &String) -> Option<&V> {
        for i in 0..self.args.len() {
            if word == &i.to_string() { return Some(&self.args[i]) }
        }
        for (var, value) in &self.vars {
            if word == var { return Some(value) }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub path: String,
    pub scopes: Vec<Scope>,
    pub global: Scope,
    pub trace: Vec<(Position, String)>
}
impl Context {
    pub fn new(path: &String) -> Self { Self { path: path.clone(), scopes: vec![Scope::new()], global: Scope::new(), trace: vec![] } }
    pub fn push(&mut self) {
        self.scopes.push(Scope::new());
    }
    pub fn pop(&mut self) -> Option<Scope> {
        self.scopes.pop()
    }
    pub fn args(&mut self, args: &Vec<V>) {
        self.scopes.last_mut().unwrap().args = args.clone();
    }
    pub fn trace(&mut self, pos: &Position) {
        self.trace.push((pos.clone(), self.path.clone()))
    }
    pub fn var(&mut self, word: &String, value: &V) -> Result<(), ()> {
        for scope in self.scopes.iter() {
            let v = scope.get(word);
            if v.is_some() { return Err(()) }
        }
        for scope in self.scopes.iter_mut().rev() {
            let res = scope.var(word, value);
            if res.is_ok() { return Ok(()) }
        }
        Err(())
    }
    pub fn set(&mut self, word: &String, value: &V) -> Result<(), ()> {
        for scope in self.scopes.iter_mut().rev() {
            let res = scope.set(word, value);
            if res.is_ok() { return Ok(()) }
        }
        Err(())
    }
    pub fn def(&mut self, word: &String, value: &V) -> Result<(), ()> {
        self.global.var(word, value)
    }
    pub fn get(&self, word: &String) -> Option<&V> {
        for scope in self.scopes.iter().rev() {
            let v = scope.get(word);
            if v.is_some() { return Some(v.unwrap()) }
        }
        self.global.get(word)
    }
    pub fn is_global(&self, word: &String) -> bool {
        self.global.get(word).is_some()
    }
}

pub fn _def(args: Vec<V>, context: &mut Context, pos: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    let addr = &args[0];
    let value = args.get(1).unwrap_or_else(|| &V::Null);
    if let V::Addr(word) = addr {
        let res = context.def(word, value);
        if res.is_err() {
            context.trace(&poses[0]);
            return Err(E::AlreadyDefined(word.clone()))
        }
        return Ok((V::Null, R::None))
    }
    context.trace(pos);
    Err(E::ExpectedType { typ: Type::Addr, recv_typ: addr.typ() })
}
pub fn _var(args: Vec<V>, context: &mut Context, pos: &Position, _: &Vec<&Position>) -> Result<(V, R), E> {
    let addr = &args[0];
    let value = args.get(1).unwrap_or_else(|| &V::Null);
    if let V::Addr(word) = addr {
        let res = context.var(word, value);
        if res.is_err() {
            context.trace(pos);
            return Err(E::AlreadyDefined(word.clone()))
        }
        return Ok((V::Null, R::None))
    }
    context.trace(pos);
    Err(E::ExpectedType { typ: Type::Addr, recv_typ: addr.typ() })
}
pub fn _set(args: Vec<V>, context: &mut Context, pos: &Position, _: &Vec<&Position>) -> Result<(V, R), E> {
    let addr = &args[0];
    let value = args.get(1).unwrap_or_else(|| &V::Null);
    if let V::Addr(word) = addr {
        if context.is_global(word) {
            context.trace(pos);
            return Err(E::Immutable(word.clone()))
        }
        let res = context.set(word, value);
        if res.is_err() {
            context.trace(pos);
            return Err(E::NotDefined(word.clone()))
        }
        return Ok((V::Null, R::None))
    }
    context.trace(pos);
    Err(E::ExpectedType { typ: Type::Addr, recv_typ: addr.typ() })
}
pub fn _get(args: Vec<V>, context: &mut Context, pos: &Position, _: &Vec<&Position>) -> Result<(V, R), E> {
    let addr = &args[0];
    if let V::Addr(word) = addr {
        let v = context.get(word);
        if v.is_some() {
            return Ok((v.unwrap().clone(), R::None))
        }
        return Ok((V::Null, R::None))
    }
    context.trace(pos);
    Err(E::ExpectedType { typ: Type::Addr, recv_typ: addr.typ() })
}
pub fn _if(args: Vec<V>, context: &mut Context, _: &Position, _: &Vec<&Position>) -> Result<(V, R), E> {
    let cond = &args[0];
    let case = args.get(1).unwrap_or_else(|| &V::Null);
    let else_case = args.get(2).unwrap_or_else(|| &V::Null);
    if cond == &V::Bool(true) {
        if let V::Closure(n, cpath) = case {
            let path = context.path.clone();
            context.path = cpath.clone();
            let res = get(n, context);
            context.path = path;
        }
        return Ok((case.clone(), R::None))
    } else if else_case != &V::Null {
        if let V::Closure(n, cpath) = else_case {
            let path = context.path.clone();
            context.path = cpath.clone();
            let res = get(n, context);
            context.path = path;
        }
        return Ok((else_case.clone(), R::None))
    }
    return Ok((V::Null, R::None))
}
pub fn _while(args: Vec<V>, context: &mut Context, _: &Position, _: &Vec<&Position>) -> Result<(V, R), E> {
    let mut cond = Type::Bool.cast(&args[0]);
    let case = args.get(1).unwrap_or_else(|| &V::Null);
    if let V::Closure(n, cpath) = &args[0] {
        let path = context.path.clone();
        context.path = cpath.clone();
        let (value, _) = get(n, context)?;
        context.path = path;
        cond = Type::Bool.cast(&value);
    }
    while cond == V::Bool(true) {
        if let V::Closure(n, cpath) = case {
            let path = context.path.clone();
            context.path = cpath.clone();
            let (value, ret) = get(n, context)?;
            context.path = path;
            if ret == R::Return { return Ok((value, ret)) }
            if ret == R::Break { return Ok((value, R::None)) }
        }
        if let V::Closure(n, cpath) = &args[0] {
            let path = context.path.clone();
            context.path = cpath.clone();
            let (value, _) = get(n, context)?;
            context.path = path;
            cond = Type::Bool.cast(&value);
        } else {
            cond = Type::Bool.cast(&args[0]);
        }
    }
    return Ok((V::Null, R::None))
}
pub fn _print(args: Vec<V>, _: &mut Context, _: &Position, _: &Vec<&Position>) -> Result<(V, R), E> {
    for i in 0..args.len() {
        print!("{}", &args[i]);
        if i < args.len() - 1 { print!(" "); }
    }
    if args.len() > 0 { println!(); }
    Ok((V::Null, R::None))
}
pub fn _add(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() == 0 { return Ok((V::Null, R::None)) }
    let mut sum = args[0].clone();
    for i in 1..args.len() {
        let v = sum.add(&args[i]);
        if v.is_none() {
            context.trace(poses[i]);
            return Err(E::BinaryOperation { type1: sum.typ(), type2: args[i].typ() })
        }
        sum = v.unwrap();
    }
    Ok((sum, R::None))
}
pub fn _sub(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() == 0 { return Ok((V::Null, R::None)) }
    if args.len() == 1 {
        let number = V::Int(0).sub(&args[0]);
        if number.is_none() {
            context.trace(poses[0]);
            return Err(E::UnaryOperation(args[0].typ()))
        }
        return Ok((number.unwrap(), R::None))
    }
    let mut sum = args[0].clone();
    for i in 1..args.len() {
        let v = sum.sub(&args[i]);
        if v.is_none() {
            context.trace(poses[i]);
            return Err(E::BinaryOperation { type1: sum.typ(), type2: args[i].typ() })
        }
        sum = v.unwrap();
    }
    Ok((sum, R::None))
}
pub fn _mul(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() == 0 { return Ok((V::Null, R::None)) }
    let mut sum = args[0].clone();
    for i in 1..args.len() {
        let v = sum.mul(&args[i]);
        if v.is_none() {
            context.trace(poses[i]);
            return Err(E::BinaryOperation { type1: sum.typ(), type2: args[i].typ() })
        }
        sum = v.unwrap();
    }
    Ok((sum, R::None))
}
pub fn _div(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() == 0 { return Ok((V::Null, R::None)) }
    let mut sum = args[0].clone();
    for i in 1..args.len() {
        let v = sum.div(&args[i]);
        if v.is_none() {
            context.trace(poses[i]);
            return Err(E::BinaryOperation { type1: sum.typ(), type2: args[i].typ() })
        }
        sum = v.unwrap();
    }
    Ok((sum, R::None))
}
pub fn _eq(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() <= 1 { return Ok((V::Bool(false), R::None)) }
    for i in 0..args.len() {
        for j in 0..args.len() {
            if args[i] != args[j] { return Ok((V::Bool(false), R::None)) }
        }
    }
    Ok((V::Bool(true), R::None))
}
pub fn _lt(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() == 0 { return Ok((V::Null, R::None)) }
    for i in 0..args.len()-1 {
        let v = args[i].lt(&args[i+1]);
        if v.is_none() {
            context.trace(poses[i]);
            return Err(E::BinaryOperation { type1: args[i].typ(), type2: args[i+1].typ() })
        }
        if v.unwrap() == V::Bool(false) { return Ok((V::Bool(false), R::None)) }
    }
    Ok((V::Bool(true), R::None))
}
pub fn _gt(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() == 0 { return Ok((V::Null, R::None)) }
    for i in 0..args.len()-1 {
        let v = args[i].gt(&args[i+1]);
        if v.is_none() {
            context.trace(poses[i]);
            return Err(E::BinaryOperation { type1: args[i].typ(), type2: args[i+1].typ() })
        }
        if v.unwrap() == V::Bool(false) { return Ok((V::Bool(false), R::None)) }
    }
    Ok((V::Bool(true), R::None))
}
pub fn _union(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() == 0 { return Ok((V::Type(Type::Union(vec![Type::Any])), R::None)) }
    let mut types: Vec<Type> = vec![];
    for i in 0..args.len() {
        if let V::Type(typ) = &args[i] {
            types.push(typ.clone());
        } else {
            context.trace(&poses[i]);
            return Err(E::ExpectedType { typ: Type::Type, recv_typ: args[i].typ() })
        }
    }
    return Ok((V::Type(Type::Union(types)), R::None))
}
pub fn _exclude(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() == 0 { return Ok((V::Type(Type::Union(vec![Type::Any])), R::None)) }
    let mut types: Vec<Type> = vec![];
    for i in 0..args.len() {
        if let V::Type(typ) = &args[i] {
            types.push(typ.clone());
        } else {
            context.trace(&poses[i]);
            return Err(E::ExpectedType { typ: Type::Type, recv_typ: args[i].typ() })
        }
    }
    return Ok((V::Type(Type::Exclusion(types)), R::None))
}
pub fn _load(args: Vec<V>, context: &mut Context, _: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args.len() == 0 { return Ok((V::Null, R::None)) }
    if let V::String(path) = &args[0] {
        let _path = context.path.clone();
        context.path = path.clone();
        runfile(path, context)?;
        context.path = _path;
        return Ok((V::Null, R::None))
    }
    context.trace(&poses[0]);
    Err(E::ExpectedType { typ: Type::String, recv_typ: args[0].typ() })
}
pub fn _assert(args: Vec<V>, context: &mut Context, pos: &Position, poses: &Vec<&Position>) -> Result<(V, R), E> {
    if args[0] == V::Bool(false) {
        context.trace(&pos);
        return Err(E::AssertError)
    }
    Ok((V::Null, R::None))
}

fn patt(pattern: Vec<Type>) -> Box<V> { Box::new(V::Pattern(pattern)) }
fn npatt() -> Box<V> { Box::new(V::Null) }

pub fn funx_context(path: &String) -> Context {
    let mut context = Context::new(path);
    let _ = context.def(&"var".to_string(),
    &V::NativFunction(patt(vec![Type::Addr, Type::Any]), _var));
    let _ = context.def(&"set".to_string(),
    &V::NativFunction(patt(vec![Type::Addr, Type::Any]), _set));
    let _ = context.def(&"def".to_string(),
    &V::NativFunction(patt(vec![Type::Addr, Type::Any]), _def));
    let _ = context.def(&"get".to_string(),
    &V::NativFunction(patt(vec![Type::Addr]), _get));

    let _ = context.def(&"if".to_string(),
    &V::NativFunction(patt(vec![Type::Bool, Type::some(), Type::Any]), _if));
    let _ = context.def(&"while".to_string(),
    &V::NativFunction(patt(vec![Type::Union(vec![Type::Bool, Type::Closure]), Type::Closure]), _while));

    let _ = context.def(&"+".to_string(),
    &V::NativFunction(npatt(), _add));
    let _ = context.def(&"-".to_string(),
    &V::NativFunction(npatt(), _sub));
    let _ = context.def(&"*".to_string(),
    &V::NativFunction(npatt(), _mul));
    let _ = context.def(&"/".to_string(),
    &V::NativFunction(npatt(), _div));

    let _ = context.def(&"=".to_string(),
    &V::NativFunction(npatt(), _eq));
    let _ = context.def(&"lt".to_string(),
    &V::NativFunction(patt(vec![Type::number(), Type::number()]), _lt));
    let _ = context.def(&"gt".to_string(),
    &V::NativFunction(patt(vec![Type::number(), Type::number()]), _gt));
    
    let _ = context.def(&"union".to_string(),
    &V::NativFunction(npatt(), _union));
    let _ = context.def(&"exclude".to_string(),
    &V::NativFunction(npatt(), _exclude));
    
    let _ = context.def(&"print".to_string(),
    &V::NativFunction(npatt(), _print));
    let _ = context.def(&"load".to_string(),
    &V::NativFunction(npatt(), _load));
    let _ = context.def(&"assert".to_string(),
    &V::NativFunction(npatt(), _assert));

    context
}