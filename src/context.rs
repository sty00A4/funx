use crate::position::*;
use crate::error::*;
use crate::values::*;
use crate::evaluator::*;

#[derive(Debug, Clone)]
pub struct Scope {
    vars: Vec<(String, V)>,
}
impl Scope {
    pub fn new() -> Self { Self { vars: vec![] } }
    pub fn var(&mut self, word: &String, value: &V) -> Result<(), ()> {
        for (var, value) in &self.vars {
            if word == var { return Err(()) }
        }
        self.vars.push((word.clone(), value.clone()));
        Ok(())
    }
    pub fn get(&self, word: &String) -> Option<&V> {
        for (var, value) in &self.vars {
            if word == var { return Some(value) }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    scopes: Vec<Scope>,
    global: Scope,
    trace: Vec<Position>
}
impl Context {
    pub fn new() -> Self { Self { scopes: vec![], global: Scope::new(), trace: vec![] } }
    pub fn trace(&mut self, pos: &Position) {
        self.trace.push(pos.clone())
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
}

pub fn _def(args: Vec<V>, context: &mut Context) -> Result<(V, R), E> {
    let addr = &args[0];
    let value = &args[1];
    if let V::Addr(word) = addr {
        let res = context.def(word, value);
        if res.is_err() {
            return Err(E::AlreadyDefined(word.clone()))
        }
        return Ok((V::Null, R::None))
    }
    Err(E::ExpectedType { typ: Type::Addr, recv_typ: addr.typ() })
}
pub fn _var(args: Vec<V>, context: &mut Context) -> Result<(V, R), E> {
    let addr = &args[0];
    let value = &args[1];
    if let V::Addr(word) = addr {
        let res = context.var(word, value);
        if res.is_err() {
            return Err(E::AlreadyDefined(word.clone()))
        }
        return Ok((V::Null, R::None))
    }
    Err(E::ExpectedType { typ: Type::Addr, recv_typ: addr.typ() })
}
pub fn _print(args: Vec<V>, context: &mut Context) -> Result<(V, R), E> {
    for i in 0..args.len() {
        print!("{}", &args[i]);
        if i < args.len() - 1 { print!(" "); }
    }
    if args.len() > 0 { println!(); }
    Ok((V::Null, R::None))
}

pub fn funx_context() -> Context {
    let mut context = Context::new();
    let _ = context.def(&"var".to_string(),
    &V::NativFunction(vec![Type::Addr, Type::Any], _var));
    let _ = context.def(&"def".to_string(),
    &V::NativFunction(vec![Type::Addr, Type::Any], _def));
    let _ = context.def(&"print".to_string(),
    &V::NativFunction(vec![], _print));
    context
}