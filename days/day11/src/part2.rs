extern crate core;

use crate::shared::OctopusGrid;
use core::utils::donwload_puzzle;
use std::slice::Iter;

pub async fn solve_part2() {
    let input: String = donwload_puzzle(11).await.unwrap();
    let mut grid = OctopusGrid::<10>::new(input.to_string());

    for i in 0..500 {
        match grid.step() {
            None => {}
            Some(count) => {
                if count == 100 {
                    println!("flashes  {}, step {}", count, i + 1);
                    break;
                }
            }
        };
    }
}
