extern crate core;

use crate::shared::{parse, Probe};
use core::utils::donwload_puzzle;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(17).await.unwrap();
    solve(input);
}

pub fn solve(input: String) {
    let bounds = parse(input);

    let mut count = 0;
    let y_max: i32 = bounds.y.min.abs() + 2;
    println!("X {} to {}", 0, bounds.x.max + 1);
    println!("Y {} to {}", -y_max, y_max);
    for x in 0..(bounds.x.max + 2) {
        for y in (-y_max)..y_max {
            let mut probe = Probe::new(x, y);
            if probe.simulate(&bounds) {
                // println!("{},{}", x, y);
                count = count + 1;
            }
        }
    }
    println!("N {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn solve_test() {
        solve("target area: x=20..30, y=-10..-5".to_string());
    }
}
