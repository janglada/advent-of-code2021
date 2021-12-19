extern crate core;

use core::utils::donwload_puzzle;
use std::borrow::BorrowMut;
use std::slice::Iter;
pub async fn solve_part2() {
    let input: String = donwload_puzzle(7).await.unwrap();
    let mut data: Vec<i32> = parse(input);
    println!("{}", data.iter().max().unwrap());
    println!("{}", data.iter().min().unwrap());
    solve(&mut data);
}

fn parse(input: String) -> Vec<i32> {
    input
        .split(',')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect()
}

fn distance(n: i32) -> i32 {
    n * (n + 1) / 2
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
fn solve(data: &mut Vec<i32>) {
    let median = median(data.borrow_mut());
    let max = data.iter().max().unwrap();
    let min = data.iter().min().unwrap();
    println!("{:?}", (*min..*max));
    println!("{} {}", *min, *max);
    let x_pos: PosTemp = (*min..*max)
        // .collect::<Vec<i32>>()
        // .iter()
        .map(|x| {
            let p: PosTemp = (x, data.iter().map(|v| distance((v - x).abs())).sum());
            p
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    println!("x_pos {}", x_pos.0);
    let fuel: i32 = data.iter().map(|v| distance((v - x_pos.0).abs())).sum();
    println!("{}", fuel)
}

type PosTemp = (i32, i32);
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
        let max = data.iter().max().unwrap();
        let min = data.iter().min().unwrap();
        println!("{:?}", (*min..*max));
        println!("{} {}", *min, *max);
        let x_pos: PosTemp = (*min..*max)
            // .collect::<Vec<i32>>()
            // .iter()
            .map(|x| {
                let p: PosTemp = (x, data.iter().map(|v| distance((v - x).abs())).sum());
                p
            })
            .min_by(|x, y| x.1.cmp(&y.1))
            .unwrap();
        println!("x_pos {}", x_pos.0);
        let fuel: i32 = data.iter().map(|v| distance((v - x_pos.0).abs())).sum();
        println!("{}", fuel)
    }
}
