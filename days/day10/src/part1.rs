extern crate core;

use crate::part1::LegalChars::OpeningAngleBracket;
use crate::part1::LegalChars::OpeningBraces;
use crate::part1::LegalChars::OpeningParenthesis;
use crate::part1::LegalChars::OpeningSquareBracket;
use std::fmt;

use crate::part1::LegalChars::ClosingAngleBracket;
use crate::part1::LegalChars::ClosingBraces;
use crate::part1::LegalChars::ClosingParenthesis;
use crate::part1::LegalChars::ClosingSquareBracket;

use core::utils::donwload_puzzle;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(10).await.unwrap();
}
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

impl LegalChars {
    #[inline]
    pub const fn is_closing(&self) -> bool {
        match self {
            ClosingSquareBracket => true,
            ClosingParenthesis => true,
            ClosingAngleBracket => true,
            ClosingBraces => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn score(&self) -> u32 {
        match self {
            ClosingSquareBracket => 57,
            ClosingParenthesis => 3,
            ClosingAngleBracket => 25137,
            ClosingBraces => 1197,
            _ => 0,
        }
    }
}

enum Mode {
    OPENING,
    CLOSING,
}
#[derive(Debug, Clone)]
struct Line(Vec<LegalChars>);

pub struct ChunksIntoIterator {
    iter: ::std::vec::IntoIter<LegalChars>,
    counters: Counters,
}

#[derive(Debug, Clone)]
pub struct Counters {
    square_bracket: i32,
    angle_bracket: i32,
    parenthesis: i32,
    braces: i32,
}

impl fmt::Display for Counters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[]: {}, <>: {}, (): {},  {{}}:{}, ",
            self.square_bracket, self.angle_bracket, self.parenthesis, self.braces
        )
    }
}

impl<'a> Iterator for ChunksIntoIterator {
    type Item = (Counters, LegalChars);
    fn next(&mut self) -> Option<(Counters, LegalChars)> {
        match self.iter.next() {
            None => None,
            Some(c) => {
                match c {
                    ClosingSquareBracket => {
                        self.counters.square_bracket = self.counters.square_bracket - 1;
                    }
                    ClosingParenthesis => {
                        self.counters.parenthesis = self.counters.parenthesis - 1;
                    }
                    ClosingAngleBracket => {
                        self.counters.angle_bracket = self.counters.angle_bracket - 1;
                    }
                    ClosingBraces => {
                        self.counters.braces = self.counters.braces - 1;
                    }

                    OpeningSquareBracket => {
                        self.counters.square_bracket = self.counters.square_bracket + 1;
                    }
                    OpeningParenthesis => {
                        self.counters.parenthesis = self.counters.parenthesis + 1;
                    }
                    OpeningAngleBracket => {
                        self.counters.angle_bracket = self.counters.angle_bracket + 1;
                    }
                    OpeningBraces => {
                        self.counters.braces = self.counters.braces + 1;
                    }
                }

                Some((self.counters.clone(), c))
            }
        }
    }
}
impl IntoIterator for Line {
    type Item = (Counters, LegalChars);
    type IntoIter = ChunksIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        ChunksIntoIterator {
            iter: self.0.into_iter(),
            counters: Counters {
                square_bracket: 0,
                braces: 0,
                angle_bracket: 0,
                parenthesis: 0,
            },
        }
    }
}
impl Line {
    // fn is_valid(&mut self) -> u32 {
    //     let last_char = self.into_iter().find(|iter_row| {
    //         iter_row.angle_bracket >= 0
    //             && iter_row.square_bracket >= 0
    //             && iter_row.braces >= 0
    //             && iter_row.parenthesis >= 0
    //     });
    //
    //     match last_char {
    //         None => 0,
    //         Some(c) => {
    //             println!("AAAAA");
    //             1
    //         }
    //     }
    // }

    fn parse(s: String) -> Line {
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

fn parse(s: String) -> Vec<Line> {
    s.split("\n").map(|l| Line::parse(l.to_string())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let line = Line::parse("[<>({}){}[([])<>]]".to_string());
        let op = line.clone().into_iter().find(|(iter_row, chars)| {
            //   println!("COUNTER {}", iter_row);
            iter_row.angle_bracket < 0
                || iter_row.square_bracket < 0
                || iter_row.braces < 0
                || iter_row.parenthesis < 0
        });
        assert!(op.is_none())
    }
    #[test]
    fn t2() {
        let line = Line::parse("{([(<{}[<>[]}>{[]{[(<()>".to_string());
        let op = line.clone().into_iter().find(|(iter_row, chars)| {
            println!("COUNTER {}", iter_row);
            iter_row.angle_bracket < 0
                || iter_row.square_bracket < 0
                || iter_row.braces < 0
                || iter_row.parenthesis < 0
        });
        assert!(op.is_some())
    }

    #[test]
    fn chunks() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

        let lines = parse(input.to_string());
        let total_score = lines
            .iter()
            .map(|l| {
                //  println!("{:?}", l);
                let last_char = l.clone().into_iter().find(|(iter_row, chars)| {
                    //   println!("COUNTER {}", iter_row);
                    iter_row.angle_bracket < 0
                        || iter_row.square_bracket < 0
                        || iter_row.braces < 0
                        || iter_row.parenthesis < 0
                });

                match last_char {
                    None => 0,
                    Some((c, lc)) => {
                        println!("CHAR {} {}", lc, lc.score());
                        lc.score()
                    }
                }
            })
            .sum::<u32>();
        println!("total_score {}", total_score)
    }
}
