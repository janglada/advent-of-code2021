extern crate core;

use crate::shared::OctopusGrid;
use core::utils::donwload_puzzle;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(11).await.unwrap();
    let mut grid = OctopusGrid::<10>::new(input.to_string());

    for i in 0..100 {
        grid.step();
    }
    println!("FLASH \n {}", grid.flash_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn masks1() {}
}
