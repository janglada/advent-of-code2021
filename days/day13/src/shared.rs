use itertools::{Itertools, MinMaxResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp, fmt};

lazy_static! {
    static ref RE: Regex = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
}

#[derive(Debug)]
pub struct DotGrid<T> {
    vec: Vec<T>,
    row: usize,
    col: usize,
}
///
/// https://stackoverflow.com/questions/13102786/two-dimensional-vectors-in-rust
///
impl<T> DotGrid<T> {
    pub fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
        assert!(vec.len() == row * col);
        Self { vec, row, col }
    }

    pub fn row(&self, row: usize) -> &[T] {
        let i = self.col * row;
        &self.vec[i..(i + self.col)]
    }

    pub fn index(&self, row: usize, col: usize) -> &T {
        let i = self.col * row;
        &self.vec[i + col]
    }

    pub fn index_mut(&mut self, row: usize, col: usize) -> &mut T {
        let i = self.col * row;
        &mut self.vec[i + col]
    }
}

pub enum Fold {
    X(usize),
    Y(usize),
}
struct MaxCounter {
    v: usize,
}
impl MaxCounter {
    fn value(&mut self, v: usize) {
        self.v = cmp::max(v, self.v);
    }
}

impl fmt::Display for DotGrid<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.row {
            for j in 0..self.col {
                if *self.index(i, j) == 1 {
                    write!(f, "#");
                } else {
                    write!(f, ".");
                }
            }
            write!(f, "\n");
        }

        Result::Ok(())
    }
}
///
///
///
///
impl DotGrid<u8> {
    ///
    ///
    ///
    pub(crate) fn fold_at(&self, fold: &Fold) -> DotGrid<u8> {
        let dots_next = match fold {
            Fold::X(pos) => {
                let rows_next: usize = self.row;
                let cols_next: usize = (self.col - 1) / 2;

                let mut dots = DotGrid::new(vec![0; rows_next * cols_next], rows_next, cols_next);
                for i in 0..rows_next {
                    for j in 0..cols_next {
                        *dots.index_mut(i, cols_next - (j + 1)) =
                            self.index(i, pos + (j + 1)) | self.index(i, pos - (j + 1));
                    }
                }
                dots
            }
            Fold::Y(pos) => {
                let rows_next: usize = (self.row - 1) / 2;
                let cols_next: usize = self.col;

                let mut dots = DotGrid::new(vec![0; rows_next * cols_next], rows_next, cols_next);
                for i in 0..rows_next {
                    for j in 0..cols_next {
                        *dots.index_mut(rows_next - (i + 1), j) =
                            self.index(pos + (i + 1), j) | self.index(pos - (i + 1), j);
                    }
                }
                dots
            }
        };
        dots_next
    }

    pub fn count_visible(&self) -> usize {
        self.vec.iter().filter(|&v| *v == 1).count()
    }
}
pub(crate) fn new_dot_grid<const N: usize, const M: usize>(
    input: String,
) -> (DotGrid<u8>, Vec<Fold>) {
    let mut iter = input.split("\n\n");

    let grid = iter.next().unwrap();

    let mut dots = DotGrid::new(vec![0; N * M], N, M);

    // let mut dots: [[u8; N]; M] = [[0; N]; M];

    let mut x_counter = MaxCounter { v: 0 };
    let mut y_counter = MaxCounter { v: 0 };

    let coords = grid.lines().for_each(|v| {
        let mut parts = v.split(",");
        let x = parts.next().unwrap().parse::<usize>().ok().unwrap();
        let y = parts.next().unwrap().parse::<usize>().ok().unwrap();

        x_counter.value(x);
        y_counter.value(y);

        *dots.index_mut(y, x) = 1;
    });

    println!("X {}", x_counter.v);
    println!("Y {}", y_counter.v);

    let folds = iter.next().unwrap();
    let ops: Vec<Fold> = folds
        .lines()
        .map(|v| {
            let mut iter = RE.captures_iter(v);
            let cap = iter.next().unwrap();
            let axis = &cap[1];
            let num = &cap[2];
            let fold_op = match axis {
                "x" => Fold::X(num.parse::<usize>().ok().unwrap()),
                "y" => Fold::Y(num.parse::<usize>().ok().unwrap()),
                _ => panic!(""),
            };
            fold_op
        })
        .collect();

    (dots, ops)
}
