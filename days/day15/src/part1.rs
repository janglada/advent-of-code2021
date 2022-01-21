extern crate core;

use crate::shared::{parse, parse2, shortest_path};
use core::utils::donwload_puzzle;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(15).await.unwrap();
    let data = parse::<100>(input.to_string());
    // let cost = min_cost(&data, 9, 9);
    // println!("cost = {:?}", cost);

    let cost = shortest_path::<100>(&data, (0, 0), (99, 99));
    println!("cost = {:?}", cost);
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

        let data = parse::<10>(input.to_string());
        // let cost = min_cost(&data, 9, 9);
        // println!("cost = {:?}", cost);

        let cost2 = shortest_path::<10>(&data, (0, 0), (9, 9));
        println!("cost2 = {:?}", cost2);
    }
}
