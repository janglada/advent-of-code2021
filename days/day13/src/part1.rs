extern crate core;

use crate::shared::new_dot_grid;
use core::utils::donwload_puzzle;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(13).await.unwrap();
    let grid = new_dot_grid::<895, 1311>(input);
    // println!("{}", grid.0);
    let mut next_grid = grid.0.fold_at(grid.1.iter().next().unwrap());
    println!("{}", next_grid.count_visible());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn masks1() {
        let input = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

        let grid = new_dot_grid::<15, 11>(input.to_string());
        println!("{}", grid.0);
        println!("\n-------------------------------\n");
        let next_grid = grid.0.fold_at(grid.1.get(0).unwrap());
        println!("{}", next_grid);
        println!("\n-------------------------------\n");
        let next_grid = next_grid.fold_at(grid.1.get(1).unwrap());
        println!("{}", next_grid);
    }
}
