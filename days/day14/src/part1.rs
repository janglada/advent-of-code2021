extern crate core;
use crate::shared::{appy_rules, parse_rules, score_by_seq};
use core::utils::donwload_puzzle;
use itertools::Itertools;
use std::cmp::Ordering;

pub async fn solve_part1() {
    // let input: String = donwload_puzzle(14).await.unwrap();
    // let (mut seq, rules) = parse_rules(&mut input.to_string());
    // for i in 0..10 {
    //     seq = appy_rules(seq, &rules);
    //     println!("loop {}, len {}", i, seq.len());
    // }
    // let score = score_by_seq(seq);
    //
    // println!("score = {:?}", score);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{appy_rules, parse_rules, score_by_seq};
    use itertools::{Itertools, MinMaxResult};
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn masks1() {
        let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

        let (mut seq, rules) = parse_rules(&mut input.to_string());
        println!("{}", seq);
        println!("{:?}", rules);
        let option = rules.get(&('C', 'C'));
        assert!(option.is_some());
        println!("{}", option.unwrap());

        for i in 0..10 {
            seq = appy_rules(seq, &rules)
        }
        println!("next_seq = {}", seq);

        let score = score_by_seq(seq);

        println!("score = {:?}", score);
    }
}
