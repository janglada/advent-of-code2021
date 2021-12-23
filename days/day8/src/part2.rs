extern crate core;

use core::utils::donwload_puzzle;

// use crate::part2::Digit::one;
use crate::part2::Digit::*;
use itertools::Itertools;

pub async fn solve_part2() {
    let _input_str: String = donwload_puzzle(8).await.unwrap();
    let inputs: Vec<Input> = parse(_input_str.to_string());
    let r = inputs
        .iter()
        .map(|_input| {
            // println!("-------------------------------------------------------");
            // println!("data {:?},  digits {:?}", _input.data, _input.digits);
            let four = _input.find_by_digit(Digit::Four);
            let seven = _input.find_by_digit(Digit::Seven);
            let one = _input.find_by_digit(Digit::One);
            let eight = _input.find_by_digit(Digit::Eight);

            // all of len =  6
            // (0 & 6 & 9 ) | 4 = nine
            let six = *_input
                .find_by_len(6)
                .iter()
                .find(|&&n| (n.v | one.v) == eight.v)
                // .map(|n| n.v)
                .unwrap();

            // 5 | 6 = 8
            let five = *_input
                .find_by_len(5)
                .iter()
                .find(|&&n| (n.v | six.v) == six.v)
                // .map(|n| n.v)
                .unwrap();

            // 2 | 4 = 8
            let two = *_input
                .find_by_len(5)
                .iter()
                .filter(|&&n| n.v != five.v)
                .find(|&&n| (n.v | four.v) == eight.v)
                // .map(|n| n.v)
                .unwrap();

            let three = *_input
                .find_by_len(5)
                .iter()
                .find(|&&n| n.v != five.v && n.v != two.v)
                .unwrap();
            // 0 xor 8 = 2 & 3 & 4 & 5
            let zero = *_input
                .find_by_len(6)
                .iter()
                .filter(|&&n| n.v != six.v)
                .find(|&&n| n.v ^ eight.v == two.v & three.v & four.v & five.v)
                .unwrap();

            let nine = *_input
                .find_by_len(6)
                .iter()
                .find(|&&n| n.v != six.v && n.v != zero.v)
                .unwrap();

            // println!("five {}", five);
            // println!("six {}", (*six).v);
            // println!("five {}", (*five).v);
            // println!("two {}", (*two).v);
            // println!("zero {}", (*zero).v);
            // println!("nine {}", (*nine).v);
            // println!();

            let row_res = _input
                .digits
                .iter()
                .map(|&v| {
                    if v == one.v {
                        '1'
                    } else if v == two.v {
                        '2'
                    } else if v == three.v {
                        '3'
                    } else if v == four.v {
                        '4'
                    } else if v == five.v {
                        '5'
                    } else if v == six.v {
                        '6'
                    } else if v == seven.v {
                        '7'
                    } else if v == eight.v {
                        '8'
                    } else if v == nine.v {
                        '9'
                    } else if v == zero.v {
                        '0'
                    } else {
                        panic!("Unknown value")
                    }
                })
                .join("")
                .parse::<i32>()
                .unwrap_or(-1);
            println!("row {}", row_res);
            row_res
        })
        .sum::<i32>();
    println!("r {}", r);
}

fn solve(inputs: &mut Input) {}
#[derive(PartialEq, Debug)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
#[derive(Debug)]
struct Number {
    v: u32,
    len: usize,
    d: Option<Digit>,
}

struct Data {
    one: Number,
    two: Number,
    three: Number,
    four: Number,
    five: Number,
    six: Number,
    seven: Number,
    eight: Number,
    nine: Number,
}

impl Number {
    fn new(v: u32, len: usize, d: Digit) -> Number {
        Number { v, len, d: Some(d) }
    }
    fn unknown(v: u32, len: usize) -> Number {
        Number { v, len, d: None }
    }

    fn set_digit(&mut self, d: Digit) {
        self.d = Some(d);
    }
}

struct Input {
    data: Vec<Number>,
    digits: Vec<u32>,
}

impl Input {
    fn find_by_len(&self, len: usize) -> Vec<&Number> {
        self.data.iter().filter(|n| n.len == len).collect()
    }

    fn find_by_value(&self, v: u32) -> Vec<&Number> {
        self.data.iter().filter(|n| n.v == v).collect()
    }

    fn find_by_digit(&self, digit: Digit) -> &Number {
        self.data
            .iter()
            .find(|n| match &(*n).d {
                None => false,
                Some(d) => *d == digit,
            })
            .unwrap()
    }
}

fn extract_digit(v: u32, len: usize) -> Number {
    match len {
        2 => Number::new(v, len, One),   // 1
        4 => Number::new(v, len, Four),  // 4
        3 => Number::new(v, len, Seven), // 7
        7 => Number::new(v, len, Eight), // 8
        _ => Number::unknown(v, len),
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
    use itertools::Itertools;

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

        // .for_each(|inp| println!("{:?} | {:?}", inp.data, inp.digits));
    }
}
