extern crate core;

use core::utils::donwload_puzzle;
use std::collections::HashMap;

pub async fn solve_part2() {
    let input = donwload_puzzle(6).await.unwrap();
    solve(&mut parse(input), 256);
}

fn parse(input: String) -> HashMap<i8, i64> {
    let mut map: HashMap<i8, i64> = HashMap::new();

    input
        .split(',')
        .map(|s| s.to_string().parse::<i8>().unwrap())
        .for_each(|n| {
            *map.entry(n).or_insert(0) += 1;
            // *map.get_mut(&n).unwrap() += 1;
        });

    map
}

pub fn solve(fishes: &mut HashMap<i8, i64>, n: u16) {
    for i in 0..n {
        let mut delta: HashMap<i8, i64> = HashMap::new();

        for (key, value) in &*fishes {
            if *key == 0 {
                *delta.entry(6).or_insert(0) += value;
                *delta.entry(0).or_insert(0) += (-value);

                // new fish
                *delta.entry(8).or_insert(0) += value;
            } else {
                *delta.entry(*key - 1).or_insert(0) += value;
                *delta.entry(*key).or_insert(0) += (-value);
            }
        }

        for (key, value) in delta {
            println!("{}", value);
            *fishes.entry(key).or_insert(0) += value;
        }
    }

    let res: i64 = fishes.values().sum();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn masks1() {
        let input = r#"3,4,3,1,2"#;

        solve(&mut parse(input.to_string()), 256);
    }
}
