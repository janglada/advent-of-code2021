extern crate core;

use core::utils::donwload_puzzle;
use std::iter;
use std::iter::IntoIterator;
pub async fn solve_part1() {
    let input: String = donwload_puzzle(9).await.unwrap();
    let heightmap = parse::<100>(input);
    heightmap.solve();
}

pub struct Heightmap<const N: usize> {
    data: [[u8; N]; N],
    size: usize,
}

pub fn parse<const N: usize>(input: String) -> Heightmap<N> {
    let mut heightmap: Heightmap<N> = Heightmap {
        data: [[0; N]; N],
        size: N,
    };

    input.split("\n").enumerate().for_each(|(i, row)| {
        row.chars().enumerate().for_each(|(j, c)| {
            heightmap.data[i][j] = c.to_string().parse::<u8>().ok().unwrap();
        })
    });

    heightmap
}

impl<const N: usize> Heightmap<N> {
    fn new() {}

    fn solve(&self) {
        let mut risk: i64 = 0;
        for (i, row) in self.data.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if self
                    .neighbours(i, j)
                    .into_iter()
                    .all(|(x, y)| self.data[x][y] > self.data[i][j])
                {
                    risk = risk + self.data[i][j] as i64 + 1;
                }
                // println!();
            }
        }
        println!("RISK {}", risk)
    }

    fn neighbours(&self, i: usize, j: usize) -> impl IntoIterator<Item = (usize, usize)> + '_ {
        let mut neigh: Vec<(usize, usize)> = Vec::new();

        if i == 0 {
            neigh.push((i + 1, j));
        } else if i == self.size - 1 {
            neigh.push((i - 1, j));
        } else {
            neigh.push((i + 1, j));
            neigh.push((i - 1, j));
        }
        if j == 0 {
            neigh.push((i, j + 1));
        } else if j == self.size - 1 {
            neigh.push((i, j - 1));
        } else {
            neigh.push((i, j + 1));
            neigh.push((i, j - 1));
        }

        neigh.into_iter()
    }
}

mod tests {
    use super::*;

    #[test]
    fn masks1() {
        let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

        let heightmap = parse::<10>(input.to_string());
        heightmap.solve();

        // .for_each(|inp| println!("{:?} | {:?}", inp.data, inp.digits));
    }
}
