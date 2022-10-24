use crate::error::*;

#[derive(Clone, PartialOrd)]
pub enum Number { Int(i64), Float(f64) }
impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
        }
    }
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => *v1 == *v2,
            (Self::Int(v1), Self::Float(v2)) => *v1 as f64 == *v2,
            (Self::Float(v1), Self::Int(v2)) => *v1 == *v2 as f64,
            (Self::Float(v1), Self::Float(v2)) => *v1 == *v2,
        }
    }
}

#[derive(Clone)]
pub enum V {
    Null, Wirldcard, Number(Number), Bool(bool), String(String),
    NativFunction(fn(Vec<V>) -> Result<V, E>)
}
impl std::fmt::Display for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::fmt::Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Wirldcard => write!(f, "_"),
            Self::Number(v) => write!(f, "{v}"),
            Self::Bool(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v}"),
            Self::NativFunction(v) => write!(f, "nativ-function:{:?}", v as *const fn(Vec<V>) -> Result<V, E>),
        }
    }
}
impl PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        if let Self::Wirldcard = self { return true }
        if let Self::Wirldcard = other { return true }
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Number(v1), Self::Number(v2)) => v1 == v2,
            (Self::Bool(v1), Self::Bool(v2)) => v1 == v2,
            (Self::String(v1), Self::String(v2)) => v1 == v2,
            (Self::NativFunction(v1), Self::NativFunction(v2)) => v1 == v2,
            _ => false
        }
    }
}