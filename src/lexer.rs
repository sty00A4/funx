use crate::position::*;
use crate::error::*;
use crate::values::*;
use crate::evaluator::*;

static WS: [&str; 4] = [" ", "\n", "\r", "\t"];
static DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
static SYMBOL: [&str; 13] = ["(", ")", "{", "}", "<", ">", "[", "]", "@", "#", "\"", "'", ";"];

#[derive(Debug, Clone, PartialEq)]
pub enum T {
    NO,
    EvalIn, EvalOut, BodyIn, BodyOut, PattIn, PattOut, VecIn, VecOut, Addr, Closure, End,
    Null, Wirldcard, Word(String), Int(i64), Float(f64), Bool(bool), String(String),
}
impl T {
    pub fn name(&self) -> &str {
        match self {
            Self::NO => "()",
            Self::EvalIn => "'('",
            Self::EvalOut => "')'",
            Self::BodyIn => "'{'",
            Self::BodyOut => "'}'",
            Self::PattIn => "'<'",
            Self::PattOut => "'>'",
            Self::VecIn => "'['",
            Self::VecOut => "']'",
            Self::Addr => "'@'",
            Self::Closure => "'#'",
            Self::End => "';'",
            Self::Null => "'null'",
            Self::Wirldcard => "'_'",
            Self::Word(_) => "word",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Bool(_) => "boolean",
            Self::String(_) => "string",
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct Token(pub T, pub Position);
impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub struct Lexer {
    path: String, text: String,
    idx: usize, ln: usize, col: usize
}
impl Lexer {
    pub fn new(path: &String, text: &String) -> Self {
        Self { path: path.clone(), text: text.clone(), idx: 0, ln: 0, col: 0 }
    }
    pub fn char(&self) -> &str {
        if self.idx >= self.text.len() { return "" }
        &self.text[self.idx..self.idx+1]
    }
    pub fn advance(&mut self) {
        if self.char() == "\n" {
            self.ln += 1; self.col = 0;
            self.idx += 1;
        } else {
            self.idx += 1; self.col += 1;
        }
    }
    pub fn pos(&self) -> (usize, usize) { (self.ln, self.col) }
    pub fn next(&mut self, context: &mut Context) -> Result<Option<Token>, E> {
        while WS.contains(&self.char()) { self.advance(); }
        if self.char() == "$" {
            self.advance();
            while self.char() != "\n" { self.advance(); }
            self.advance();
            if self.char() == "" { return Ok(None) }
            while WS.contains(&self.char()) { self.advance(); }
        }
        if self.char() == "" { return Ok(None) }
        let (ln_start, col_start) = self.pos();
        if DIGITS.contains(&self.char()) {
            let mut number = String::new();
            let mut dot = false;
            while DIGITS.contains(&self.char()) || self.char() == "." {
                if self.char() == "." {
                    if dot { break } else { dot = true; }
                    number.push_str(self.char());
                    self.advance();
                } else {
                    number.push_str(self.char());
                    self.advance();
                }
            }
            if dot {
                return Ok(Some(
                    Token(T::Float(number.parse::<f64>().unwrap()),
                    Position::new(ln_start..self.ln, col_start..self.col))
                ))
            }
            return Ok(Some(
                Token(
                    T::Int(number.parse::<i64>().unwrap()),
                    Position::new(ln_start..self.ln, col_start..self.col)
                )
            ))
        }
        if self.char() == "\"" || self.char() == "'" {
            let end = if self.char() == "\"" { "\"" } else { "'" };
            let mut string = String::new();
            self.advance();
            while self.char() != end {
                string.push_str(self.char());
                self.advance();
            }
            self.advance();
            if self.char() == "n" { self.advance(); } else { string = string.replace("\n", "").replace("\r", ""); }
            string = string.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
            return Ok(Some(
                Token(
                    T::String(string),
                    Position::new(ln_start..self.ln, col_start..self.col)
                )
            ))
        }
        if self.char() == "@" {
            self.advance();
            return Ok(Some(
                Token(T::Addr, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == "#" {
            self.advance();
            return Ok(Some(
                Token(T::Closure, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == ";" {
            self.advance();
            return Ok(Some(
                Token(T::End, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == "(" {
            self.advance();
            return Ok(Some(
                Token(T::EvalIn, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == ")" {
            self.advance();
            return Ok(Some(
                Token(T::EvalOut, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == "{" {
            self.advance();
            return Ok(Some(
                Token(T::BodyIn, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == "}" {
            self.advance();
            return Ok(Some(
                Token(T::BodyOut, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == "<" {
            self.advance();
            return Ok(Some(
                Token(T::PattIn, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == ">" {
            self.advance();
            return Ok(Some(
                Token(T::PattOut, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == "[" {
            self.advance();
            return Ok(Some(
                Token(T::VecIn, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        if self.char() == "]" {
            self.advance();
            return Ok(Some(
                Token(T::VecOut, Position::new(ln_start..self.ln, col_start..self.col))
            ))
        }
        let mut word = String::new();
        while !WS.contains(&self.char()) && !SYMBOL.contains(&self.char()) && self.char() != "" {
            word.push_str(self.char());
            self.advance();
        }
        match word.as_str() {
            "null" => Ok(Some(Token(T::Null, Position::new(ln_start..self.ln, col_start..self.col)))),
            "_" => Ok(Some(Token(T::Wirldcard, Position::new(ln_start..self.ln, col_start..self.col)))),
            "true" | "false" => Ok(Some(Token(T::Bool(word == "true"), Position::new(ln_start..self.ln, col_start..self.col)))),
            _ => return Ok(Some(Token(T::Word(word), Position::new(ln_start..self.ln, col_start..self.col))))
        }
    }
}

pub fn lex(path: &String, text: &String, context: &mut Context) -> Result<Vec<Token>, E> {
    let mut lexer = Lexer::new(path, text);
    let mut tokens: Vec<Token> = vec![];
    loop {
        let token = lexer.next(context)?;
        if token.is_none() { break }
        tokens.push(token.unwrap());
    }
    Ok(tokens)
}