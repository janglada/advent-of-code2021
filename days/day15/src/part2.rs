extern crate core;

use crate::shared::{parse2, shortest_path};
use core::utils::donwload_puzzle;
use std::slice::Iter;

pub async fn solve_part2() {
    let input: String = donwload_puzzle(15).await.unwrap();
    let data = parse2::<500, 5>(input.to_string());
    // let cost = min_cost(&data, 9, 9);
    // println!("cost = {:?}", cost);

    let cost = shortest_path::<500>(&data, (0, 0), (499, 499));
    println!("cost  = {:?}", cost);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{min_cost, parse, shortest_path};
    use std::cmp;
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_counter() {
        let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

        let data = parse2::<50, 5>(input.to_string());
        // let cost = min_cost(&data, 9, 9);
        // println!("cost = {:?}", cost);
        for i in 0..50 {
            for j in 0..50 {
                print!("{}", data[i][j]);
            }
            print!("\n");
        }

        let cost = shortest_path::<50>(&data, (0, 0), (49, 49));
        println!("cost  = {:?}", cost);
    }
}
