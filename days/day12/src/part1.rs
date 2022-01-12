extern crate core;

use crate::shared::{AdjacencyList, TreeNodeType};
use core::utils::donwload_puzzle;
use itertools::{cloned, Itertools};
use std::cmp;
use std::collections::HashMap;
use std::ops::Range;
use std::vec::IntoIter;

pub async fn solve_part1() {
    // let mut adj = AdjacencyList::new();
    // donwload_puzzle(12).await.unwrap().lines().for_each(|s| {
    //     let mut split_iter = s.clone().split("-");
    //     let n1 = TreeNodeType::from_text(split_iter.next().unwrap());
    //     let n2 = TreeNodeType::from_text(split_iter.next().unwrap());
    //     adj.add(n1, n2);
    // });
    //
    // // let root = adj.get_start();
    // let mut res = Some(vec![vec![TreeNodeType::Start]]);
    //
    // while res.as_ref().unwrap().into_iter().any(|v| {
    //     if let Some(TreeNodeType::End) = v.last() {
    //         false
    //     } else {
    //         // Destructure failed. Change to the failure case.
    //         true
    //     }
    // }) {
    //     res = adj.map_until2(&mut res.unwrap());
    // }
    //
    // dbg!("{}", res.unwrap().len());
}

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
        let input = r#"mx-IQ
mx-HO
xq-start
start-HO
IE-qc
HO-end
oz-xq
HO-ni
ni-oz
ni-MU
sa-IE
IE-ni
end-sa
oz-sa
MU-start
MU-sa
oz-IE
HO-xq
MU-xq
IE-end
MU-mx"#;
        let mut adj = AdjacencyList::new();
        input.lines().for_each(|s| {
            let mut split_iter = s.split("-");
            let n1 = TreeNodeType::from_text(split_iter.next().unwrap());
            let n2 = TreeNodeType::from_text(split_iter.next().unwrap());
            adj.add(n1, n2);
        });
        let root = adj.get_start();
        let mut res = Some(vec![vec![TreeNodeType::Start]]);

        while res.as_ref().unwrap().into_iter().any(|v| {
            if let Some(TreeNodeType::End) = v.last() {
                false
            } else {
                // Destructure failed. Change to the failure case.
                true
            }
        }) {
            res = adj.map_until2(&mut res.unwrap());
        }

        dbg!("{}", res.unwrap().len());
        // let root = adj.get_start();
    }
}
