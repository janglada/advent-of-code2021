extern crate core;
use ansi_term::Colour::Green;
use core::utils::donwload_puzzle;
use itertools::Itertools;
use std::fmt;
struct Position {
    pub num: u32,
    is_marked: bool,
}

impl Position {
    pub fn default(num: u32) -> Position {
        Position {
            num,
            is_marked: false,
        }
    }

    ///
    ///
    ///
    pub fn mark_and_get(&mut self, val: u32) -> u8 {
        if self.num == val {
            self.is_marked = true;
        }
        if self.is_marked {
            1
        } else {
            0
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_marked {
            write!(
                f,
                "{}",
                Green.paint(format!("{:0>2} ", self.num.to_string()))
            );
        } else {
            write!(f, "{}", format!("{:0>2} ", self.num.to_string()));
        }

        Result::Ok(())
    }
}

struct Board {
    pub board_id: u32,
    pub bingo: bool,
    positions: Vec<Position>,
}
impl Board {
    pub fn default(id: u32, nums: Vec<Position>) -> Board {
        Board {
            board_id: id,
            bingo: false,
            positions: nums,
        }
    }

    pub fn bingo(&mut self) {
        self.bingo = true;
    }

    pub fn apply(&mut self, value: u32) -> Option<u32> {
        let mut bingo = false;
        for i in 0..5 {
            let mut col: u8 = 0;
            let mut row: u8 = 0;

            for j in 0..5 {
                col = col + self.positions[5 * i + j].mark_and_get(value);
                row = row + self.positions[5 * j + i].mark_and_get(value);
                // println!(
                //     "SETTING {} {}, values {}   {} ",
                //     i,
                //     j,
                //     self.positions[5 * i + j].num,
                //     self.positions[5 * j + i].num
                // )
            }
            if !bingo {
                bingo = col == 5 || row == 5;
            }
        }
        if bingo {
            let row: Vec<u32> = self
                .positions
                .iter()
                .filter(|&p| !p.is_marked)
                .map(|p| p.num)
                .collect();
            println!("{:?}", row);

            let score: u32 = row.iter().sum::<u32>();
            println!("score!!! {}", score);
            println!("BINGO!!! {}", score * value);
            return Some(self.board_id);
        }
        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.positions.iter().enumerate().for_each(|(i, p)| {
            write!(f, "{}", p);
            if i % 5 == 4 {
                write!(f, "\n");
            }
        });
        Result::Ok(())
    }
}

struct Game {
    boards: Vec<Board>,
    nums: Vec<u32>,
}

impl Game {
    pub fn bingo(&mut self) -> Option<u32> {
        for n in self.nums.iter() {
            println!("-------------------------------  {}", n);
            for b in self.boards.iter_mut().filter(|mut board| !board.bingo) {
                match b.apply(*n) {
                    None => {}
                    Some(n) => {
                        b.bingo();
                        println!("board {}  bingo!", b.board_id);
                        // return Some(n);
                    }
                }
            }

            // println!("{}", self);
        }
        None
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in self.boards.iter() {
            write!(f, "{}\n", b);
        }
        Result::Ok(())
    }
}

///
///
///
///
fn init_game(input: String) -> Game {
    let mut lines_iter = input.split("\n\n");

    let numbers: Vec<u32> = lines_iter
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    // println!("{:?}", numbers);
    let mut board_id: u32 = 0;
    let boards: Vec<Board> = lines_iter
        .map(|l| {
            let x: Vec<Position> = l
                .lines()
                .flat_map(|n| n.split_whitespace())
                .map(|n| n.parse().unwrap())
                .map(|n| Position::default(n))
                .collect();
            board_id = board_id + 1;
            Board::default(board_id, x)
        })
        .collect();

    Game {
        nums: numbers,
        boards,
    }
}

///
///
///
///
pub async fn solve_part1() {
    let input = donwload_puzzle(4).await.unwrap();
    let mut game: Game = init_game(input);
    // game.boards.iter().for_each(|b| println!("{}", b));
    // let mut game: Game = init_game(input.to_string());
    game.bingo();
    println!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks1() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
        let mut game: Game = init_game(input.to_string());
        game.bingo();
    }

    #[test]
    fn test_bingo() {
        let input = r#"7,4

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
        let mut game: Game = init_game(input.to_string());
        game.bingo();
    }
}
