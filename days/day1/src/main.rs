extern crate core;

use core::utils::donwload_puzzle;
use itertools::Itertools;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    let input: String = donwload_puzzle(1).await.unwrap();
    let mut v: Vec<u32> = input
        .lines()
        .map(|line| line.to_string().parse().unwrap())
        .collect();

    let part1 = v.iter().tuple_windows().filter(|(a, b)| b > a).count();

    println!("{}", part1);

    let part2 = v
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();
    println!("{}", part2);
    Ok(())
}
