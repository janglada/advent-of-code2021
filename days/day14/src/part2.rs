extern crate core;

use crate::shared::{
    count_pairs_for_seq, pair_count_2_letter, parse_rules, score_by_counts, update_pair_freq,
    PairCounts, Rules,
};
use core::utils::donwload_puzzle;
use std::collections::HashMap;
use std::slice::Iter;

pub async fn solve_part2() {
    let input: String = donwload_puzzle(14).await.unwrap();
    // println!("{}", input);
    let (mut seq, rules) = parse_rules(&mut input.to_string());

    let mut pairs: PairCounts = count_pairs_for_seq(seq);
    //println!("counts = {:?}", counts);
    for i in 0..40 {
        pairs = update_pair_freq(&pairs, &rules);
    }

    let mut letter_count = pair_count_2_letter(pairs).clone();
    *letter_count.entry('B').or_insert(0) += 1;

    // let score = score(seq);

    let score = score_by_counts(letter_count.clone());
    println!("letter_counts = {:?}", letter_count);
    println!("score = {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        appy_rules, count_letters_for_seq, count_pairs_for_seq, hashmap_compare,
        pair_count_2_letter, parse_rules, score_by_counts, score_by_seq, update_pair_freq,
        PairCounts,
    };
    use itertools::{Itertools, MinMaxResult};
    use std::cmp;
    use std::collections::HashMap;
    use std::iter::once;

    #[test]
    fn test_counter() {
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

        let mut pairs = count_pairs_for_seq(seq);
        pairs = update_pair_freq(&pairs, &rules);
        let mut letter_count = pair_count_2_letter(pairs).clone();
        let mut expected_letter_count = count_letters_for_seq("NCNBCHB".to_string());
        *letter_count.entry('B').or_insert(0) += 1;

        println!(
            "{:?} == {:?}? {}",
            letter_count,
            expected_letter_count,
            hashmap_compare(&letter_count, &expected_letter_count)
        );
    }

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

        let mut pairs: PairCounts = count_pairs_for_seq(seq);
        //println!("counts = {:?}", counts);
        for i in 0..40 {
            pairs = update_pair_freq(&pairs, &rules);
        }

        let mut letter_count = pair_count_2_letter(pairs).clone();
        *letter_count.entry('B').or_insert(0) += 1;

        // let score = score(seq);

        let score = score_by_counts(letter_count.clone());
        println!("letter_counts = {:?}", letter_count);
        println!("score = {}", score);
    }
}
