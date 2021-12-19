extern crate core;

use core::utils::donwload_puzzle;
use day7::part1::solve_part1;
use day7::part2::solve_part2;
use itertools::Itertools;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    solve_part1().await;
    solve_part2().await;

    Ok(())
}
