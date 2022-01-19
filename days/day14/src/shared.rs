use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::hash::Hash;

pub type Rules = HashMap<(char, char), char>;
pub type LetterCounts = HashMap<char, u64>;
pub type PairCounts = HashMap<(char, char), u64>;

pub fn parse_rules(input: &mut String) -> (String, Rules) {
    let mut split = input.split("\n\n");
    let input_seq = split.next().unwrap();

    let mut rules = Rules::new();
    split.next().unwrap().lines().for_each(|line| {
        let mut row_spit = line.split(" -> ");
        let mut seq = row_spit.next().unwrap().chars();
        rules.insert(
            (seq.next().unwrap(), seq.next().unwrap()),
            row_spit.next().unwrap().chars().next().unwrap(),
        );
    });

    (input_seq.to_string(), rules)
}

pub fn appy_rules(seq: String, rules: &Rules) -> String {
    let last = seq.chars().last().unwrap();
    let mut next_seq: String = seq
        .chars()
        .tuple_windows()
        .flat_map(|(a, b)| vec![a, *rules.get(&(a, b)).unwrap()])
        .collect();
    next_seq.push(last);

    next_seq
}

pub fn update_pair_freq(letter_counts: &PairCounts, rules: &Rules) -> PairCounts {
    let mut next_letter_counts = letter_counts.clone();
    letter_counts.keys().for_each(|c| {
        let next_char = rules.get(c).unwrap();
        // println!("pair {:?}", c);

        // let prev1: &u64 = letter_counts.get(&(c.0, *next_char)).unwrap_or(&0);
        // let prev2: &u64 = letter_counts.get(&(*next_char, c.1)).unwrap_or(&0);
        let prev3: &u64 = letter_counts.get(c).unwrap_or(&0);
        *next_letter_counts.entry((c.0, *next_char)).or_insert(0) += prev3;
        *next_letter_counts.entry((*next_char, c.1)).or_insert(0) += prev3;
        *next_letter_counts.entry(*c).or_insert(0) -= prev3;
        // *next_letter_counts.entry(*c).or_insert(0) -= 1;
    });

    next_letter_counts
}

pub fn count_pairs_for_seq(seq: String) -> PairCounts {
    seq.chars()
        .tuple_windows()
        .fold(PairCounts::new(), |mut map, c| {
            println!("PAIRS {:?}", c);
            *map.entry(c).or_insert(0) += 1;
            map
        })
}

pub fn count_letters_for_seq(seq: String) -> LetterCounts {
    seq.chars().fold(LetterCounts::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}

pub fn pair_count_2_letter(pair_counts: PairCounts) -> LetterCounts {
    pair_counts
        .iter()
        .filter(|&c| c.1 > &0)
        //.flat_map(|e| once(tup.0).chain(once(tup.1)))
        .fold(LetterCounts::new(), |mut map, c| {
            *map.entry(c.0 .0).or_insert(0) += c.1;
            // *map.entry(c.0.1).or_insert(0) += c.1;
            map
        })
}

pub fn score_by_seq(seq: String) -> u64 {
    let letter_counts = count_letters_for_seq(seq);
    match letter_counts.iter().minmax_by_key(|&e| e.1) {
        MinMaxResult::NoElements => {
            panic!("!!!")
        }
        MinMaxResult::OneElement(_) => {
            panic!("!!!")
        }
        MinMaxResult::MinMax(min, max) => {
            // println!("{:?}", min);
            // println!("{:?}", max);
            // println!("res {}", max.1 - min.1);
            max.1 - min.1
        }
    }
}

pub fn score_by_counts(letter_counts: LetterCounts) -> u64 {
    match letter_counts.iter().minmax_by_key(|&e| e.1) {
        MinMaxResult::NoElements => {
            panic!("!!!")
        }
        MinMaxResult::OneElement(_) => {
            panic!("!!!")
        }
        MinMaxResult::MinMax(min, max) => {
            // println!("{:?}", min);
            // println!("{:?}", max);
            // println!("res {}", max.1 - min.1);
            max.1 - min.1
        }
    }
}

pub fn hashmap_compare<T: Eq + Hash, U: std::cmp::PartialEq>(
    map1: &HashMap<T, U>,
    map2: &HashMap<T, U>,
) -> bool {
    map1.len() == map2.len()
        && map1.keys().all(|k| map2.contains_key(k))
        && map1
            .keys()
            .all(|k| map1.get(k).unwrap() == map2.get(k).unwrap())
}
