extern crate core;

use day10::part1::solve_part1;
use day10::part2::solve_part2;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    // solve_part1().await;
    solve_part2().await;

    Ok(())
}
