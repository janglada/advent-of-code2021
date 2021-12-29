extern crate core;
use super::shared::*;
// use crate::shared::LegalChars::OpeningAngleBracket;
// use crate::shared::LegalChars::OpeningBraces;
// use crate::part1::LegalChars::OpeningParenthesis;
// use crate::part1::LegalChars::OpeningSquareBracket;
use std::fmt;

// use crate::part1::LegalChars::ClosingAngleBracket;
// use crate::part1::LegalChars::ClosingBraces;
// use crate::part1::LegalChars::ClosingParenthesis;
// use crate::part1::LegalChars::ClosingSquareBracket;

use core::utils::donwload_puzzle;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(10).await.unwrap();
    let total_score = parse(input)
        .iter()
        .map(|l| {
            //  println!("{:?}", l);
            l.clone()
                .into_iter()
                .find(|&score| {
                    //   println!("COUNTER {}", iter_row);
                    score != 0
                })
                .unwrap_or(0 as u32)
        })
        .sum::<u32>();

    println!("total_score {}", total_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let line = Line::parse("[<>({}){}[([])<>]]".to_string());
        let op = line.clone().into_iter().find(|&score| {
            //   println!("COUNTER {}", iter_row);
            score != 0
        });
        assert!(op.is_none())
    }
    #[test]
    fn t2() {
        let line = Line::parse("{([(<{}[<>[]}>{[]{[(<()>".to_string());
        let op = line.clone().into_iter().find(|&score| score != 0);
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
                l.clone()
                    .into_iter()
                    .find(|&score| {
                        //   println!("COUNTER {}", iter_row);
                        score != 0
                    })
                    .unwrap_or(0 as u32)
            })
            .sum::<u32>();
        println!("total_score {}", total_score)
    }
}
