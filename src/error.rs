use std::fmt::format;

use crate::position::*;
use crate::values::*;
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
}
impl E {
    pub fn display(&self, path: &String, context: &Context) -> String {
        let mut string: String = format!("{self}");
        string.push_str("\n");
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
        }
    }
}