use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Position(pub Range<usize>, pub Range<usize>);
impl Position {
    pub fn new(ln: Range<usize>, col: Range<usize>) -> Self {
        Self(ln,  col)
    }
}