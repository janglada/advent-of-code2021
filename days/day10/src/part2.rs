extern crate core;

use super::shared::*;
use core::utils::donwload_puzzle;
use itertools::Itertools;
use std::fmt;
use std::slice::Iter;
/*
pub struct ChunkChar {
    kind: ChunkCharType,
    closing: bool,
}

impl ChunkChar {
    fn new_square_bracket(closing: bool) -> ChunkChar {
        ChunkChar {
            kind: ChunkCharType::SquareBracket,
            closing,
        }
    }
    fn new_parenthesis(closing: bool) -> ChunkChar {
        ChunkChar {
            kind: ChunkCharType::Parenthesis,
            closing,
        }
    }
    fn new_angle_bracket(closing: bool) -> ChunkChar {
        ChunkChar {
            kind: ChunkCharType::AngleBracket,
            closing,
        }
    }
    fn new_braces(closing: bool) -> ChunkChar {
        ChunkChar {
            kind: ChunkCharType::Braces,
            closing,
        }
    }

    pub const fn score(&self) -> u32 {
        if self.closing {
            match self.kind {
                ChunkCharType::SquareBracket => 57,
                ChunkCharType::Parenthesis => 3,
                ChunkCharType::AngleBracket => 25137,
                ChunkCharType::Braces => 1197,
            }
        } else {
            0
        }
    }
}

impl fmt::Display for ChunkChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ChunkCharType::SquareBracket => {
                if self.closing {
                    write!(f, "]")
                } else {
                    write!(f, "[")
                }
            }
            ChunkCharType::Parenthesis => {
                if self.closing {
                    write!(f, ")")
                } else {
                    write!(f, "(")
                }
            }
            ChunkCharType::AngleBracket => {
                if self.closing {
                    write!(f, ">")
                } else {
                    write!(f, "<")
                }
            }
            ChunkCharType::Braces => {
                if self.closing {
                    write!(f, "}}")
                } else {
                    write!(f, "{{")
                }
            }
        }
    }
}

pub enum ChunkCharType {
    SquareBracket,
    Parenthesis,
    AngleBracket,
    Braces,
}
*/

pub async fn solve_part2() {
    let input: String = donwload_puzzle(10).await.unwrap();
    let lines = clean_up(parse(input));

    // let peekable = lines.iter().peekable();
    println!("LINES {}", lines.len());
    let res = solve(lines);
    println!("res {}", res);
}
pub fn solve(lines: Vec<Line>) -> u64 {
    let v: Vec<u64> = lines
        .into_iter()
        .map(|line| {
            // pairwise iteration
            // let mut v = Vec::new();
            let mut stack: Vec<LegalChars> = Vec::new();
            for curr in line.0.iter() {
                let prev: &LegalChars = match stack.last() {
                    None => {
                        stack.push(curr.clone());
                        continue;
                    }
                    Some(c) => c,
                };

                match curr {
                    LegalChars::ClosingSquareBracket => match prev {
                        LegalChars::OpeningSquareBracket => {
                            stack.pop();
                        }
                        _ => {}
                    },
                    LegalChars::ClosingParenthesis => match prev {
                        LegalChars::OpeningParenthesis => {
                            stack.pop();
                        }
                        _ => {}
                    },
                    LegalChars::ClosingAngleBracket => match prev {
                        LegalChars::OpeningAngleBracket => {
                            stack.pop();
                        }
                        _ => {}
                    },
                    LegalChars::ClosingBraces => match prev {
                        LegalChars::OpeningBraces => {
                            stack.pop();
                        }
                        _ => {}
                    },
                    a => {
                        stack.push(*a);
                    }
                }

                // if !prev.is_closing() && (*curr).is_closing() {
                //     if matches!(*prev, LegalChars::OpeningAngleBracket)
                //         && matches!(curr, LegalChars::ClosingAngleBracket)
                //         || matches!(*prev, LegalChars::OpeningSquareBracket)
                //             && matches!(curr, LegalChars::ClosingSquareBracket)
                //         || matches!(*prev, LegalChars::OpeningParenthesis)
                //             && matches!(curr, LegalChars::ClosingParenthesis)
                //         || matches!(*prev, LegalChars::OpeningBraces)
                //             && matches!(curr, LegalChars::ClosingBraces)
                //     {
                //         // println!("{} {}", prev, curr);
                //     }
                // } else {
                //     stack.push(curr.clone());
                // }
                // println!("{} {}", prev, curr);
                // v.push((a, b));
            }
            let mut cloned = Line(stack.clone());
            println!("{} ", cloned);
            cloned.0.iter().rev().fold(0u64, |sum, v| {
                let score: u64 = match v {
                    LegalChars::OpeningSquareBracket => 2,
                    LegalChars::OpeningParenthesis => 1,
                    LegalChars::OpeningAngleBracket => 4,
                    LegalChars::OpeningBraces => 3,
                    c => {
                        panic!("FOUND CHAR {}", c);
                    }
                };
                sum * 5 + score
            })
        })
        .sorted()
        .collect();
    let middle = v.len() / 2;
    println!("middle {}", middle);

    *v.get(middle).unwrap()
}

pub fn clean_up(lines: Vec<Line>) -> Vec<Line> {
    lines
        .into_iter()
        .filter(|l| {
            l.clone()
                .into_iter()
                .find(|&score| {
                    //   println!("COUNTER {}", iter_row);
                    score != 0
                })
                .is_none()
        })
        .collect()
}
/*
#[derive(Debug, Clone)]
pub struct Chunk(Vec<ChunkChar>);

pub fn parse_line(s: String) -> Chunk {
    Chunk {
        0: s.chars()
            .map(|c| match c {
                '{' => ChunkChar::new_braces(false),
                '}' => ChunkChar::new_braces(false),

                '[' => ChunkChar::new_square_bracket(false),
                ']' => ChunkChar::new_square_bracket(false),

                '(' => ChunkChar::new_parenthesis(false),
                ')' => ChunkChar::new_parenthesis(false),

                '<' => ChunkChar::new_angle_bracket(false),
                '>' => ChunkChar::new_angle_bracket(false),
                _ => {
                    panic!("Invalid")
                }
            })
            .collect(),
    }
}
*/
#[cfg(test)]
mod tests {
    use super::*;

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

        let lines = clean_up(parse(input.to_string()));
        let res = solve(lines);
        println!("res {}", res);
    }
}
