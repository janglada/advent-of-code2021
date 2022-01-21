#![feature(generic_const_exprs)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn parse<const N: usize>(input: String) -> [[u32; N]; N] {
    let mut data = [[0u32; N]; N];
    input.lines().enumerate().for_each(|x| {
        x.1.chars().enumerate().for_each(|c| {
            data[x.0][c.0] = c.1.to_digit(10).unwrap();
        })
    });
    data
}

pub fn parse2<const N: usize, const R: usize>(input: String) -> [[u32; N]; N] {
    let mut data = [[0u32; N]; N];
    let side = N / R;

    input.lines().enumerate().for_each(|x| {
        x.1.chars().enumerate().for_each(|c| {
            for i in 0..R {
                for j in 0..R {
                    let mut v = c.1.to_digit(10).unwrap() + i as u32 + j as u32;
                    if v > 9 {
                        v = v % 9;
                    }
                    data[i * side + x.0][j * side + c.0] = v;
                }
            }
        })
    });
    data
}

/* Returns cost of minimum cost path
from (0,0) to (m, n) in mat[R][C]*/
pub fn min_cost<const N: usize>(cost: &[[u32; N]; N], m: isize, n: isize) -> u32 {
    // println!("MIN COST {} {}", n, m);
    return if n < 0 || m < 0 || n >= (N as isize) || m >= (N as isize) {
        u32::MAX
    } else if m == 0 && n == 0 {
        0
    } else {
        cost[m as usize][n as usize]
            + cmp::min(
                min_cost(cost, m - 1, n - 1),
                cmp::min(min_cost(cost, m - 1, n), min_cost(cost, m, n - 1)),
            )
    };
}

type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Position,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: u32,
}

fn safe_coords<const N: isize>(x: isize, y: isize) -> Option<Position> {
    if x >= 0 && x < N && y >= 0 && y < N {
        Some((
            usize::try_from(x).unwrap_or(0),
            usize::try_from(y).unwrap_or(0),
        ))
    } else {
        None
    }
}

fn neighbours<const N: usize>(p: Position) -> impl IntoIterator<Item = Position> {
    let mut neigh: Vec<Position> = Vec::new();
    let i = p.0;
    let j = p.1;
    if i == 0 {
        neigh.push((i + 1, j));
    } else if i == N - 1 {
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

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
pub fn shortest_path<const N: usize>(
    cost_matrix: &[[u32; N]; N],
    start: Position,
    goal: Position,
) -> Option<u32> {
    /* 8 possible moves */

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: [[u32; N]; N] = [[u32::MAX; N]; N];

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start.0][start.1] = 0 as u32;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position.0][position.1] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        neighbours::<N>(position)
            .into_iter()
            .for_each(|next_position| {
                // let next_position: Position = (position.0 + dx[i], position.1 + dy[i]);

                let next = State {
                    cost: cost + cost_matrix[next_position.0][next_position.1],
                    position: next_position,
                };

                // If so, add it to the frontier and continue
                if next.cost < dist[next.position.0][next.position.1] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist[next.position.0][next.position.1] = next.cost;
                }
            })
    }

    // Goal not reachable
    None
}
