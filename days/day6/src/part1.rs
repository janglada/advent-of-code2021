extern crate core;

use core::utils::donwload_puzzle;
use std::iter::once;

#[derive(Clone)]
pub struct Fish {
    days: u8,
}

impl Fish {
    fn new(days: u8) -> Fish {
        Fish { days }
    }
    fn newBorn() -> Fish {
        Fish { days: 8 }
    }

    fn reset(&mut self) {
        self.days = 6;
    }

    fn step(&mut self) -> Option<Fish> {
        if self.days == 0 {
            self.reset();
            return Some(Fish::newBorn());
        } else {
            self.days = self.days - 1;
        }
        None
    }
}

fn parse(input: String) -> Vec<Fish> {
    input
        .split(',')
        .map(|s| s.to_string().parse::<u8>().unwrap())
        .map(|n| Fish::new(n))
        .collect()
}

pub async fn solve_part1() {
    let input = donwload_puzzle(6).await.unwrap();
    solve(&mut parse(input), 256);
}

pub fn solve(fishes: &mut Vec<Fish>, n: u16) {
    for i in 0..n {
        for i in 0..fishes.len() {
            match fishes[i].step() {
                None => {}
                Some(c) => {
                    fishes.push(c);
                }
            }
        }
        println!("i={} {} fishes", i, fishes.len());
    }
    println!("{}", fishes.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn masks1() {
        let input = r#"3,4,3,1,2"#;

        solve(&mut parse(input.to_string()), 80);
    }
}
