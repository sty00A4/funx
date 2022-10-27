use std::fmt::format;
use std::fs;
use crate::position::*;
use crate::values::*;
use crate::context::*;
use crate::lexer::*;
use crate::parser::*;
use crate::evaluator::*;

#[derive(Debug, Clone, PartialEq)]
pub enum E {
    TargetNotFound(String),
    FileNotFound(String),
    Char(String),
    UnexpectedToken(T),
    HeadOperation(V),
    ExpectedType { typ: Type, recv_typ: Type },
    AlreadyDefined(String),
    BinaryOperation { type1: Type, type2: Type },
    UnaryOperation(Type),
}
impl E {
    pub fn display(&self, context: &Context) -> String {
        let text = fs::read_to_string(&context.path).unwrap_or_else(|_|"".to_string());
        let mut string: String = format!("{self}");
        string.push_str("\n");
        if text.len() > 0 {
            let lines: Vec<&str> = text.split("\n").collect();
            for pos in context.trace.iter() {
                string.push_str(format!("{}:{}:{} - {}:{}\n",
                &context.path, pos.0.start + 1, pos.1.start + 1, pos.0.end + 1, pos.1.end + 1).as_str());
                string.push_str(lines[pos.0.start..pos.0.end + 1].join("\n").as_str());
                string.push_str("\n");
            }
        } else {
            string.push_str(text.as_str());
            string.push_str("\n");
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
            Self::ExpectedType{ typ, recv_typ } => write!(f, "ERROR: expected {typ} but got {recv_typ}"),
            Self::AlreadyDefined(word) => write!(f, "ERROR: word {word} is already defined"),
            Self::BinaryOperation{ type1, type2 } => write!(f, "ERROR: illegal operation between {type1} and {type2}"),
            Self::UnaryOperation(typ) => write!(f, "ERROR: illegal operation on {typ}"),
        }
    }
}