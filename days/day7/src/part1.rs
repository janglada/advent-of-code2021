extern crate core;

use core::utils::donwload_puzzle;
use std::borrow::BorrowMut;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(7).await.unwrap();
    let mut data: Vec<i32> = parse(input);
    let median = median(data.borrow_mut());
    println!("{}", median);
    let fuel: i32 = data.iter().map(|v| (v - median).abs()).sum();
    println!("{}", fuel)
}

fn parse(input: String) -> Vec<i32> {
    input
        .split(',')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect()
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }
}

fn mean(numbers: &Vec<i32>) -> f32 {
    let sum: i32 = numbers.iter().sum();

    sum as f32 / numbers.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn masks1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let mut data: Vec<i32> = parse(input.to_string());
        let median = median(data.borrow_mut());

        let fuel: i32 = data.iter().map(|v| (v - median).abs()).sum();
        println!("{}", fuel)
    }
}
