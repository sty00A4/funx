use crate::position::*;
use crate::values::*;
use crate::lexer::*;
use crate::parser::*;
use crate::evaluator::*;

#[derive(Debug, Clone, PartialEq)]
pub enum E {
    TargetNotFound(String),
    FileNotFound { path: String, pos: Position },
    Char { char: String, pos: Position },
    UnexpectedToken { token: T, pos: Position },
    HeadOperation { value: V, pos: Position },
    ExpectedType { typ: Type, recv_typ: Type, pos: Position },
}
impl std::fmt::Display for E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TargetNotFound(path) => write!(f, "ERROR: target file {path:?} could not be found"),
            Self::FileNotFound{ path, pos } => write!(f, "ERROR: file {path:?} could not be found"),
            Self::Char{ char, pos } => write!(f, "ERROR: bad character {char:?}"),
            Self::UnexpectedToken{ token, pos } => write!(f, "ERROR: unexpected {}", token.name()),
            Self::HeadOperation{ value, pos } => write!(f, "ERROR: unexpected {} as head operation", value.typ()),
            Self::ExpectedType{ typ, recv_typ, pos } => write!(f, "ERROR: unexpected {typ} but got {recv_typ}"),
        }
    }
}