extern crate core;

use crate::shared::{AdjacencyList, TreeNodeType};
use core::utils::donwload_puzzle;
use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::ops::Range;
use std::vec::IntoIter;

pub async fn solve_part1() {}

fn children_of(n: i32) -> IntoIter<i32> {
    let v = match n {
        1 => vec![2, 3],
        2 => vec![4, 5, 6],
        3 => vec![7],
        _ => panic!("AAAA"),
    };

    v.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::{cloned, concat};
    use std::iter::once;
    #[test]
    fn xx() {
        ///             1
        ///           /   \
        ///          2     3
        ///        / | \    \
        ///       4  5  6    7
        ///
        /// vec![1, 2, 4], vec![1, 2, 5], vec![1, 2, 6], vec![1, 3, 7]
        ///
        let root = vec![1];
        let children_of_1 = vec![2, 3];
        let children_of_2 = vec![4, 5, 6];
        let children_of_3 = vec![7];

        let mut paths = vec![root];

        let vectors: Vec<Vec<i32>> = paths
            .iter_mut()
            .flat_map(|v| {
                children_of(*v.last().unwrap()).map(move |c| concat(vec![v.clone(), vec![c]]))
            })
            .flat_map(|v| {
                children_of(*v.last().unwrap()).map(move |c| concat(vec![v.clone(), vec![c]]))
            })
            .collect();

        println!("{:?}", vectors);
    }

    #[test]
    fn tp1() {
        let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        let mut adj = AdjacencyList::new();
        input.lines().for_each(|s| {
            let mut split_iter = s.split("-");
            let n1 = TreeNodeType::from_text(split_iter.next().unwrap());
            let n2 = TreeNodeType::from_text(split_iter.next().unwrap());
            adj.add(n1, n2);
        })
    }
}
