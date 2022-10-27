use crate::position::*;
use crate::error::*;
use crate::values::*;
use crate::context::*;
use crate::lexer::*;

#[derive(Clone, PartialEq)]
pub enum N {
    Eval(Vec<Node>), Body(Vec<Node>), Pattern(Vec<Node>), Vector(Vec<Node>),
    Addr(Box<Node>), Arg(Box<Node>), Closure(Box<Node>),
    Null, Wirldcard, Word(String), Int(i64), Float(f64), Bool(bool), String(String), Type(Type)
}
impl std::fmt::Debug for N {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eval(nodes) => write!(f, "({})", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Body(nodes) => write!(f, "{{{}}}", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("; ")),
            Self::Pattern(nodes) => write!(f, "<{}>", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Vector(nodes) => write!(f, "[{}]", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Addr(node) => write!(f, "@{node}"),
            Self::Arg(node) => write!(f, "%{node}"),
            Self::Closure(node) => write!(f, "#{node}"),
            Self::Null => write!(f, "null"),
            Self::Wirldcard => write!(f, "_"),
            Self::Word(v) => write!(f, "{v}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::Bool(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v:?}"),
            Self::Type(v) => write!(f, "{v}"),
        }
    }
}
impl std::fmt::Display for N {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Clone, PartialEq)]
pub struct Node(pub N, pub Position);
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub idx: usize
}
impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self { Self { tokens: tokens.clone(), idx: 0 } }
    pub fn advance(&mut self) { self.idx += 1; }
    pub fn token_pos(&self) -> &Token {
        if self.idx >= self.tokens.len() { return &Token(T::NO, Position(0..0, 0..0)) }
        &self.tokens[self.idx]
    }
    pub fn token(&self) -> &T {
        if self.idx >= self.tokens.len() { return &T::NO }
        &self.tokens[self.idx].0
    }
    pub fn pos(&self) -> &Position {
        if self.idx >= self.tokens.len() { return &self.tokens.last().unwrap().1 }
        &self.tokens[self.idx].1
    }
    pub fn parse(&mut self, context: &mut Context) -> Result<Node, E> {
        let start = self.pos().clone();
        let mut body_nodes: Vec<Node> = vec![];
        while self.token() != &T::NO {
            let start_node = self.pos().clone();
            let mut nodes: Vec<Node> = vec![];
            while self.token() != &T::End && self.token() != &T::NO {
                let node = self.next(context)?;
                nodes.push(node);
            }
            if self.token() == &T::End { self.advance(); }
            body_nodes.push(Node(
                N::Eval(nodes),
                Position::new(start_node.0.start..self.pos().0.end, start_node.1.start..self.pos().1.end)
            ));
        }
        if body_nodes.len() == 1 {
            return Ok(body_nodes[0].clone())
        }
        return Ok(Node(N::Body(body_nodes), Position::new(start.0.start..self.pos().0.end, start.1.start..self.pos().1.end)))
    }
    pub fn next(&mut self, context: &mut Context) -> Result<Node, E> {
        let start = self.pos().clone();
        if self.token() == &T::EvalIn {
            self.advance();
            let mut nodes: Vec<Node> = vec![];
            while self.token() != &T::EvalOut {
                let node = self.next(context)?;
                nodes.push(node);
            }
            self.advance();
            return Ok(Node(N::Eval(nodes), Position::new(start.0.start..self.pos().0.end, start.1.start..self.pos().1.end)))
        }
        if self.token() == &T::BodyIn {
            self.advance();
            let mut body_nodes: Vec<Node> = vec![];
            while self.token() != &T::BodyOut {
                let start_node = self.pos().clone();
                let mut nodes: Vec<Node> = vec![];
                while self.token() != &T::End && self.token() != &T::BodyOut {
                    let node = self.next(context)?;
                    nodes.push(node);
                }
                if self.token() == &T::End { self.advance(); }
                body_nodes.push(Node(
                    N::Eval(nodes),
                    Position::new(start_node.0.start..self.pos().0.end, start_node.1.start..self.pos().1.end)
                ));
            }
            self.advance();
            return Ok(Node(N::Body(body_nodes), Position::new(start.0.start..self.pos().0.end, start.1.start..self.pos().1.end)))
        }
        if self.token() == &T::PattIn {
            self.advance();
            let mut nodes: Vec<Node> = vec![];
            while self.token() != &T::PattOut {
                let node = self.next(context)?;
                nodes.push(node);
            }
            self.advance();
            return Ok(Node(N::Pattern(nodes), Position::new(start.0.start..self.pos().0.end, start.1.start..self.pos().1.end)))
        }
        if self.token() == &T::VecIn {
            self.advance();
            let mut nodes: Vec<Node> = vec![];
            while self.token() != &T::VecOut {
                let node = self.next(context)?;
                nodes.push(node);
            }
            self.advance();
            return Ok(Node(N::Pattern(nodes), Position::new(start.0.start..self.pos().0.end, start.1.start..self.pos().1.end)))
        }
        if self.token() == &T::Addr {
            self.advance();
            let node = self.next(context)?;
            let pos = node.1.clone();
            return Ok(Node(N::Addr(Box::new(node)), Position::new(start.0.start..pos.0.end, start.1.start..pos.1.end)))
        }
        if self.token() == &T::Arg {
            self.advance();
            let node = self.next(context)?;
            let pos = node.1.clone();
            return Ok(Node(N::Arg(Box::new(node)), Position::new(start.0.start..pos.0.end, start.1.start..pos.1.end)))
        }
        if self.token() == &T::Closure {
            self.advance();
            let node = self.next(context)?;
            let pos = node.1.clone();
            return Ok(Node(N::Closure(Box::new(node)), Position::new(start.0.start..pos.0.end, start.1.start..pos.1.end)))
        }
        if let Token(T::Int(v), pos) = self.token_pos() {
            let node = Ok(Node(N::Int(v.clone()), pos.clone()));
            self.advance();
            return node
        }
        if let Token(T::Float(v), pos) = self.token_pos() {
            let node = Ok(Node(N::Float(v.clone()), pos.clone()));
            self.advance();
            return node
        }
        if let Token(T::Bool(v), pos) = self.token_pos() {
            let node = Ok(Node(N::Bool(v.clone()), pos.clone()));
            self.advance();
            return node
        }
        if let Token(T::String(v), pos) = self.token_pos() {
            let node = Ok(Node(N::String(v.clone()), pos.clone()));
            self.advance();
            return node
        }
        if let Token(T::Type(v), pos) = self.token_pos() {
            let node = Ok(Node(N::Type(v.clone()), pos.clone()));
            self.advance();
            return node
        }
        if let Token(T::Word(v), pos) = self.token_pos() {
            let node = Ok(Node(N::Word(v.clone()), pos.clone()));
            self.advance();
            return node
        }
        if let Token(T::Null, pos) = self.token_pos() {
            let node = Ok(Node(N::Null, pos.clone()));
            self.advance();
            return node
        }
        if let Token(T::Wirldcard, pos) = self.token_pos() {
            let node = Ok(Node(N::Wirldcard, pos.clone()));
            self.advance();
            return node
        }
        context.trace(self.pos());
        Err(E::UnexpectedToken(self.token().clone()))
    }
}

pub fn parse(tokens: &Vec<Token>, context: &mut Context) -> Result<Node, E> {
    let mut parser = Parser::new(tokens);
    parser.parse(context)
}