use std::fs;
use crate::values::*;
use crate::context::*;
use crate::lexer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum E {
    TargetNotFound(String),
    FileNotFound(String),
    Char(String),
    UnexpectedToken(T),
    HeadOperation(V),
    ExpectedType { typ: Type, recv_typ: Type },
    NotDefined(String),
    AlreadyDefined(String),
    Immutable(String),
    BinaryOperation { type1: Type, type2: Type },
    UnaryOperation(Type),
    PatternMissmatch { pattern1: V, pattern2: V },
    ExpectedLen { len: usize, recv_len: usize },
    AssertError,
}
impl E {
    pub fn display(&self, context: &Context) -> String {
        let mut string: String = format!("{self}");
        string.push_str("\n");
        for (pos, path) in context.trace.iter() {
            let text = fs::read_to_string(path).unwrap_or_else(|_|"".to_string());
            if text.len() > 0 {
                let lines: Vec<&str> = text.split("\n").collect();
                string.push_str(format!("{}:{}:{} - {}:{}\n",
                &context.path, pos.0.start + 1, pos.1.start + 1, pos.0.end + 1, pos.1.end + 1).as_str());
                string.push_str(lines[pos.0.start..pos.0.end + 1].join("\n").as_str());
                string.push_str("\n");
            } else {
                string.push_str(text.as_str());
                string.push_str("\n");
            }
        }
        string
    }
}
impl std::fmt::Display for E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TargetNotFound(path) => write!(f, "ERROR: target file {path:?} could not be found"),
            Self::FileNotFound(path) => write!(f, "ERROR: file {path:?} could not be found"),
            Self::Char(char) => write!(f, "ERROR: bad character {char:?}"),
            Self::UnexpectedToken(token) => write!(f, "ERROR: unexpected {}", token.name()),
            Self::HeadOperation(value) => write!(f, "ERROR: unexpected {} as head operation", value.typ()),
            Self::ExpectedType{ typ, recv_typ } => write!(f, "ERROR: expected type {typ} but got type {recv_typ}"),
            Self::NotDefined(word) => write!(f, "ERROR: word {word} is not defined"),
            Self::AlreadyDefined(word) => write!(f, "ERROR: word {word} is already defined"),
            Self::Immutable(word) => write!(f, "ERROR: word {word} is immutable"),
            Self::BinaryOperation{ type1, type2 } => write!(f, "ERROR: illegal operation between type {type1} and type {type2}"),
            Self::UnaryOperation(typ) => write!(f, "ERROR: illegal operation on type {typ}"),
            Self::PatternMissmatch { pattern1, pattern2 } => write!(f, "ERROR: pattern {pattern1} does not match {pattern2}"),
            Self::ExpectedLen { len, recv_len } => write!(f, "ERROR: expected pattern to be at least of length {len} not {recv_len}"),
            Self::AssertError => write!(f, "ERROR: assertion is false"),
        }
    }
}