extern crate core;
extern crate peg;

use core::utils::donwload_puzzle;
use itertools::Itertools;
use std::cmp;

peg::parser! {
  grammar lines_parser() for str {
    rule line() -> Line
      = v1:vertex() " -> " v2:vertex() {
            Line::new(v1,v2)
        }

    pub rule number() -> u32
       = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    pub rule vertex() -> Vertex
      = x:number() "," y:number() {
           Vertex::new(x, y)
        }

    pub rule list() -> Vec<Line>
      = l:line() ** "\n"  { l }
  }
}
#[derive(Debug)]
pub struct Vertex {
    pub x: u32,
    pub y: u32,
}
#[derive(Debug)]
pub struct Line {
    p1: Vertex,
    p2: Vertex,
    a: f64,
    b: f64,
}
impl Vertex {
    fn new(x: u32, y: u32) -> Vertex {
        Vertex { x, y }
    }
}
impl Line {
    fn new(p1: Vertex, p2: Vertex) -> Line {
        let a = (p2.y as f64 - p1.y as f64) / (p2.x as f64 - p1.x as f64);
        let b = p1.y as f64 - a * (p1.x as f64);

        // println!("y = {}x + {}", a, b);

        Line { p1, p2, a, b }
    }
    pub fn is_in_line(&self, x: u32, y: u32) -> bool {
        let max_x = cmp::max(self.p2.x, self.p1.x);
        let min_x = cmp::min(self.p2.x, self.p1.x);
        let max_y = cmp::max(self.p2.y, self.p1.y);
        let min_y = cmp::min(self.p2.y, self.p1.y);

        if max_x >= x && min_x <= x && max_y >= y && min_y <= y {
            if self.a.is_finite() {
                ((y as f64) - (self.a * (x as f64) + self.b)).abs() < f64::EPSILON
            } else {
                self.p1.x == x
            }
        } else {
            false
        }
    }
}

pub fn parse(input: String) -> Vec<Line> {
    let lines: Vec<Line> = lines_parser::list(input.as_str()).ok().unwrap();

    lines
}
///
///
///
///
pub async fn solve_part1() {
    let input = donwload_puzzle(5).await.unwrap();
    let lines: Vec<Line> = parse(input);
    solve(lines);
    // game.boards.iter().for_each(|b| println!("{}", b));
    // let mut game: Game = init_game(input.to_string());

    println!()
}

pub fn solve(lines: Vec<Line>) -> u32 {
    let mut lines: Vec<Line> = lines
        .into_iter()
        // .filter(|l| l.p1.x == l.p2.x || l.p1.y == l.p2.y)
        .collect();
    println!("{}", lines.len());
    // let mut points = HashMap::new();

    let max_x = lines
        .iter()
        .map(|l| cmp::max(l.p1.x, l.p2.x))
        .max()
        .unwrap();
    let max_y = lines
        .iter()
        .map(|l| cmp::max(l.p1.y, l.p2.y))
        .max()
        .unwrap();
    let mut global_counter = 0;
    for i in 0..(max_x + 1) {
        for j in 0..(max_y + 1) {
            let n = lines.iter().filter(|l| l.is_in_line(i, j)).count();
            // if n == 0 {
            //     print!(".");
            // } else {
            //     print!("{}", n);
            // }
            // println!("{} {}", i, j);
            if n >= 2 {
                global_counter = global_counter + 1;
            }
        }
        // print!("\n");
    }
    println!("N = {}", global_counter);
    global_counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn masks1() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

        solve(parse(input.to_string()));
    }
}
