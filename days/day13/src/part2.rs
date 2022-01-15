extern crate core;

use crate::shared::new_dot_grid;
use core::utils::donwload_puzzle;
use std::slice::Iter;

pub async fn solve_part2() {
    let input: String = donwload_puzzle(13).await.unwrap();
    let grid = new_dot_grid::<895, 1311>(input);
    // println!("{}", grid.0);
    let mut next_grid = grid.0;
    grid.1.iter().for_each(|f| {
        next_grid = next_grid.fold_at(f);
    });

    println!("{}", next_grid);
}
