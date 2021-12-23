extern crate core;

use core::utils::donwload_puzzle;
use std::slice::Iter;
// use crate::part2::Digit::One;
use crate::part2::Digit::*;

pub async fn solve_part2() {
    let input: String = donwload_puzzle(8).await.unwrap();
}
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

struct Number {
    v: u32,
    d: Option<Digit>,
}

impl Number {
    fn new(v: u32, d: Digit) -> Number {
        Number { v, d: Some(d) }
    }
    fn unknown(v: u32) -> Number {
        Number { v, d: None }
    }

    fn set_digit(&mut self, d: Digit) {
        self.d = Some(d);
    }
}

struct Input {
    data: Vec<Number>,
    digits: Vec<u32>,
}

fn extract_digit(v: u32, len: usize) -> Number {
    match len {
        2 => Number::new(v, One),   // 1
        4 => Number::new(v, Four),  // 4
        3 => Number::new(v, Seven), // 7
        7 => Number::new(v, Eight), // 8
        _ => Number::unknown(v),
    }
}
fn to_int(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 1 << 2,
        'c' => 1 << 3,
        'd' => 1 << 4,
        'e' => 1 << 5,
        'f' => 1 << 6,
        'g' => 1 << 7,
        // _ => {
        //     panic!("unexpected pattern ")
        // }
        // _ => {}
        _ => {
            panic!("unexpected pattern {}", c)
        }
    }
}

fn parse(input: String) -> Vec<Input> {
    input
        .split('\n')
        .map(|row| {
            let mut iter = row.trim().split("|");
            let data: Vec<Number> = iter
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|e| {
                    // e.chars().count()
                    let v = e
                        .chars()
                        .map(|c| to_int(c))
                        .fold(0u32, |sum, val| sum | val);
                    let n = e.chars().count();

                    extract_digit(v, n)
                })
                .collect();
            // iter = row.trim().split("|");
            let digit: Vec<u32> = iter
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|e| {
                    // e.chars().count()
                    e.chars()
                        .map(|c| to_int(c))
                        .fold(0u32, |sum, val| sum | val)
                })
                .collect();

            Input {
                data: data,
                digits: digit,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn masks1() {
        let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

        let inputs: Vec<Input> = parse(input.to_string());

        let num = inputs.iter().for_each(|inp| println!("{:?}", inp.digits));

        let num = inputs
            .iter()
            .flat_map(|inp| inp.digits.iter())
            .filter(|&&n| match n {
                2 => true, // 1
                4 => true, // 4
                3 => true, // 7
                7 => true, // 8
                _ => false,
            })
            .count();
        dbg!(num);
        // .for_each(|inp| println!("{:?} | {:?}", inp.data, inp.digits));
    }
}
