use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum LegalChars {
    ClosingSquareBracket,
    ClosingParenthesis,
    ClosingAngleBracket,
    ClosingBraces,

    OpeningSquareBracket,
    OpeningParenthesis,
    OpeningAngleBracket,
    OpeningBraces,
}

impl LegalChars {
    #[inline]
    pub const fn is_closing(&self) -> bool {
        match self {
            LegalChars::ClosingSquareBracket => true,
            LegalChars::ClosingParenthesis => true,
            LegalChars::ClosingAngleBracket => true,
            LegalChars::ClosingBraces => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn score(&self) -> u32 {
        match self {
            LegalChars::ClosingSquareBracket => 57,
            LegalChars::ClosingParenthesis => 3,
            LegalChars::ClosingAngleBracket => 25137,
            LegalChars::ClosingBraces => 1197,
            _ => 0,
        }
    }
}
impl fmt::Display for LegalChars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LegalChars::ClosingSquareBracket => write!(f, "]"),
            LegalChars::ClosingParenthesis => write!(f, ")"),
            LegalChars::ClosingAngleBracket => write!(f, ">"),
            LegalChars::ClosingBraces => write!(f, "}}"),
            LegalChars::OpeningSquareBracket => write!(f, "["),
            LegalChars::OpeningParenthesis => write!(f, "("),
            LegalChars::OpeningAngleBracket => write!(f, "<"),
            LegalChars::OpeningBraces => write!(f, "{{"),
        }
    }
}

//

#[derive(Debug, Clone)]
pub struct Line(pub(crate) Vec<LegalChars>);
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.0.iter() {
            write!(f, "{}", c);
        }

        Ok(())
    }
}

impl Line {
    pub fn parse(s: String) -> Line {
        Line {
            0: s.chars()
                .map(|c| match c {
                    '{' => LegalChars::OpeningBraces,
                    '}' => LegalChars::ClosingBraces,

                    '[' => LegalChars::OpeningSquareBracket,
                    ']' => LegalChars::ClosingSquareBracket,

                    '(' => LegalChars::OpeningParenthesis,
                    ')' => LegalChars::ClosingParenthesis,

                    '<' => LegalChars::OpeningAngleBracket,
                    '>' => LegalChars::ClosingAngleBracket,
                    _ => {
                        panic!("Invalid")
                    }
                })
                .collect(),
        }
    }
}

pub struct ChunksIntoIterator {
    pub(crate) iter: ::std::vec::IntoIter<LegalChars>,
    pub(crate) stack: Vec<LegalChars>,
}

impl ChunksIntoIterator {
    pub fn get_stack(&self) -> Vec<LegalChars> {
        self.stack.clone()
    }
}

impl<'a> Iterator for ChunksIntoIterator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        match self.iter.next() {
            None => None,
            Some(c) => {
                return match c {
                    LegalChars::ClosingSquareBracket => match self.stack.last() {
                        Some(LegalChars::OpeningSquareBracket) => {
                            Some(self.stack.pop().unwrap().score())
                        }
                        _ => Some(LegalChars::ClosingSquareBracket.score()),
                    },
                    LegalChars::ClosingParenthesis => match self.stack.last() {
                        Some(LegalChars::OpeningParenthesis) => {
                            Some(self.stack.pop().unwrap().score())
                        }
                        _ => Some(LegalChars::ClosingParenthesis.score()),
                    },
                    LegalChars::ClosingAngleBracket => match self.stack.last() {
                        Some(LegalChars::OpeningAngleBracket) => {
                            Some(self.stack.pop().unwrap().score())
                        }
                        _ => Some(LegalChars::ClosingAngleBracket.score()),
                    },
                    LegalChars::ClosingBraces => match self.stack.last() {
                        Some(LegalChars::OpeningBraces) => Some(self.stack.pop().unwrap().score()),
                        _ => Some(LegalChars::ClosingBraces.score()),
                    },
                    a => {
                        self.stack.push(a);
                        Some(0)
                    }
                }
            }
        }
    }
}
impl IntoIterator for Line {
    type Item = u32;
    type IntoIter = ChunksIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        ChunksIntoIterator {
            iter: self.0.into_iter(),
            stack: vec![],
        }
    }
}

pub fn parse(s: String) -> Vec<Line> {
    s.split("\n").map(|l| Line::parse(l.to_string())).collect()
}
