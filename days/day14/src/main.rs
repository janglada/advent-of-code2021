extern crate core;

use core::utils::donwload_puzzle;
use day14::part1::solve_part1;
use day14::part2::solve_part2;
use itertools::Itertools;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    solve_part1().await;
    solve_part2().await;

    Ok(())
}
