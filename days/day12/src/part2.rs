extern crate core;

use core::utils::donwload_puzzle;
use std::slice::Iter;

pub async fn solve_part2() {
    let input: String = donwload_puzzle(12).await.unwrap();
}
