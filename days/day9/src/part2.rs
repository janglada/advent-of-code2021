extern crate core;

use ansi_term::Color::Green;
use ansi_term::Color::Purple;
use ansi_term::Color::Red;
use ansi_term::Color::Yellow;
use ansi_term::Color::{Blue, White};
use core::utils::donwload_puzzle;
use itertools::Itertools;
use std::slice::Iter;
use std::{fmt, iter};

pub async fn solve_part2() {
    let input: String = donwload_puzzle(9).await.unwrap();
    let mut heightmap = parse::<100, 100>(input);
    heightmap.solve();
}

#[derive(Copy, Clone, Default)]
struct HeightPoint {
    height: u8,
    basin_id: Option<u32>,
}

///
///
///
pub struct Heightmap<const M: usize, const N: usize> {
    data: [[HeightPoint; N]; M],
}

///
///
///
pub fn parse<const M: usize, const N: usize>(input: String) -> Heightmap<M, N> {
    // println!("{} {}", M, N);

    let mut heightmap: Heightmap<M, N> = Heightmap {
        data: [[HeightPoint::default(); N]; M],
    };

    input.split("\n").enumerate().for_each(|(i, row)| {
        row.chars().enumerate().for_each(|(j, c)| {
            heightmap.data[i][j] = HeightPoint {
                height: c.to_string().parse::<u8>().ok().unwrap(),
                basin_id: None,
            }
        })
    });
    // println!("ROWS {}", heightmap.data.len());
    // println!("ROWS {:?}", heightmap.data[0]);
    heightmap
}

type Position = (usize, usize);
type Height = (usize, usize);

impl<const M: usize, const N: usize> Heightmap<M, N> {
    ///
    ///
    ///
    fn solve(&mut self) {
        let sinks = self.locate_sinks();
        let mut basin_id: u32 = 0;
        let n = sinks
            .iter()
            .map(|&p| {
                basin_id = basin_id + 1;
                self.basin_of(p, basin_id)
            })
            // .inspect(|v| println!("->{}  {:?}", v.len(), v))
            .map(|basin| basin.len())
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .fold(1, |sum, item| sum * item);

        println!("n = {}", n);
    }

    ///
    ///
    ///
    fn locate_sinks(&self) -> Vec<Position> {
        let mut sinks: Vec<Position> = Vec::<Position>::new();
        for (i, row) in self.data.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if self
                    .neighbours((i, j))
                    .into_iter()
                    .all(|(x, y)| self.data[x][y].height > self.data[i][j].height)
                {
                    sinks.push((i, j));
                }
                // println!();
            }
        }
        sinks
    }
    // fn basin_of2<'a>(&self, pos: Position) -> Box<dyn Iterator<Item = &Position> + 'a> {
    //     Box::new(
    //         iter::once(pos).chain(
    //             self.neighbours(pos)
    //                 .into_iter()
    //                 .map(|n| self.basin_of2(n))
    //                 .flatten(),
    //         ),
    //     )
    // }
    ///
    ///
    ///
    fn basin_of<'a>(&mut self, pos: Position, basin_id: u32) -> Vec<Position> {
        // let mut result: Vec<Position> = Basin::new();
        // result.push(pos);
        // println!("FILTERED ({} {}) {}", pos.0, pos.1, self.data[pos.0][pos.1]);

        let neighbours = self.basin_neighbours(pos, basin_id).clone();

        iter::once(pos)
            .chain(
                neighbours
                    .into_iter()
                    .map(|p| self.basin_of(p, basin_id))
                    .flatten(),
            )
            .collect()
    }

    //  impl Node {
    // pub fn values<'a>(&'a self) -> Box<dyn Iterator<Item = &i32> + 'a> {
    //     Box::new(
    //         self.values
    //             .iter()
    //             .chain(self.children.iter().map(|n| n.values()).flatten()),
    //     )
    // }
    //}
    fn neighbours<'a>(&'a self, p: Position) -> impl IntoIterator<Item = Position> + 'a {
        let mut neigh: Vec<Position> = Vec::new();
        let i = p.0;
        let j = p.1;
        if i == 0 {
            neigh.push((i + 1, j));
        } else if i == M - 1 {
            neigh.push((i - 1, j));
        } else {
            neigh.push((i + 1, j));
            neigh.push((i - 1, j));
        }
        if j == 0 {
            neigh.push((i, j + 1));
        } else if j == N - 1 {
            neigh.push((i, j - 1));
        } else {
            neigh.push((i, j + 1));
            neigh.push((i, j - 1));
        }

        neigh.into_iter()
    }

    fn add_neighbours(&mut self, i: usize, j: usize, ref_value: u8) -> Option<Position> {
        let mut h = self.data[i][j];
        if h.basin_id.is_none() {
            if h.height < 9 && h.height > ref_value {
                return Some((i, j));
            }
            h.basin_id = Some(1);
        }

        None
    }

    /// Neighbours
    ///
    ///
    fn basin_neighbours(&mut self, p: Position, basin_id: u32) -> Vec<Position> {
        let mut neigh: Vec<Position> = Vec::new();
        let i = p.0;
        let j = p.1;
        let mut ref_position = &mut self.data[i][j];
        let ref_value = ref_position.height;
        ref_position.basin_id = Some(basin_id);

        if i == 0 {
            // self.add_neighbours(i + 1, j, ref_value)
            //     .and_then(|p| neigh.push(p););
            let mut h = &mut self.data[i + 1][j];
            if h.basin_id.is_none() {
                if h.height < 9 && h.height > ref_value {
                    neigh.push((i + 1, j));
                    h.basin_id = Some(basin_id);
                }
            }
        } else if i == M - 1 {
            let mut h = &mut self.data[i - 1][j];
            if h.basin_id.is_none() {
                if h.height < 9 && h.height > ref_value {
                    neigh.push((i - 1, j));
                    h.basin_id = Some(basin_id);
                }
            }
        } else {
            let mut h = &mut self.data[i + 1][j];
            if h.basin_id.is_none() {
                if h.height < 9 && h.height > ref_value {
                    neigh.push((i + 1, j));
                    h.basin_id = Some(basin_id);
                }
            }
            let mut h = &mut self.data[i - 1][j];
            if h.basin_id.is_none() {
                if h.height < 9 && h.height > ref_value {
                    neigh.push((i - 1, j));
                    h.basin_id = Some(basin_id);
                }
            }
        }

        if j == 0 {
            let mut h = &mut self.data[i][j + 1];
            if h.basin_id.is_none() {
                h.basin_id = Some(basin_id);
                if h.height < 9 && h.height > ref_value {
                    neigh.push((i, j + 1));
                }
            }
        } else if j == N - 1 {
            let mut h = &mut self.data[i][j - 1];
            if h.basin_id.is_none() {
                if h.height < 9 && h.height > ref_value {
                    neigh.push((i, j - 1));
                    h.basin_id = Some(basin_id);
                }
            }
        } else {
            let mut h = &mut self.data[i][j + 1];
            if h.basin_id.is_none() {
                if h.height < 9 && h.height > ref_value {
                    neigh.push((i, j + 1));
                    h.basin_id = Some(basin_id);
                }
            }
            let mut h = &mut self.data[i][j - 1];
            if h.basin_id.is_none() {
                if h.height < 9 && h.height > ref_value {
                    neigh.push((i, j - 1));
                    h.basin_id = Some(basin_id);
                }
            }
        }

        neigh
    }
}

impl<const M: usize, const N: usize> fmt::Display for Heightmap<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..M {
            for j in 0..N {
                if Some(1) == self.data[i][j].basin_id {
                    write!(f, "{}", Green.paint(format!("{} ", self.data[i][j].height)));
                } else if Some(2) == self.data[i][j].basin_id {
                    write!(f, "{}", Red.paint(format!("{} ", self.data[i][j].height)));
                } else if Some(3) == self.data[i][j].basin_id {
                    write!(
                        f,
                        "{}",
                        Yellow.paint(format!("{} ", self.data[i][j].height))
                    );
                } else if Some(4) == self.data[i][j].basin_id {
                    write!(
                        f,
                        "{}",
                        Purple.paint(format!("{} ", self.data[i][j].height))
                    );
                } else {
                    write!(f, "{}", White.paint(format!("{} ", self.data[i][j].height)));
                }
            }
            write!(f, "\n");
        }
        Ok(())
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

        let mut heightmap = parse::<5, 10>(input.to_string());
        heightmap.solve();
        println!("{}", heightmap)

        // .for_each(|inp| println!("{:?} | {:?}", inp.data, inp.digits));
    }
}
