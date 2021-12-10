extern crate core;

use core::utils::donwload_puzzle;
use itertools::Itertools;
use std::io::Error;

#[derive(Debug, Clone, Copy)]
enum InstructionKind {
    Forward,
    Down,
    Up,
}
#[derive(Debug, Clone, Copy)]
struct Instruction {
    kind: InstructionKind,
    amount: u32,
}

type Instructions = Vec<Instruction>;

#[derive(Debug, Clone, Copy, Default)]
struct State {
    x: u32,
    y: u32,
    aim: u32,
}

impl State {
    fn applyPart1(&mut self, ins: &Instruction) {
        match ins.kind {
            InstructionKind::Forward => self.x = self.x + ins.amount,
            InstructionKind::Up => self.y = self.y - ins.amount,
            InstructionKind::Down => self.y = self.y + ins.amount,
        }
    }

    fn applyPart2(&mut self, ins: &Instruction) {
        match ins.kind {
            InstructionKind::Forward => {
                self.x = self.x + ins.amount;
                self.y = self.y + self.aim * ins.amount;
            }
            InstructionKind::Up => self.aim = self.aim - ins.amount,
            InstructionKind::Down => self.aim = self.aim + ins.amount,
        }
    }

    fn value(self) -> u32 {
        self.x * self.y
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    let input: String = donwload_puzzle(2).await.unwrap();
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let mut tokens = l.split(' ');
            Instruction {
                kind: match tokens.next() {
                    Some(tok) => match tok {
                        "forward" => InstructionKind::Forward,
                        "down" => InstructionKind::Down,
                        "up" => InstructionKind::Up,
                        _ => panic!("unknown instruction kind {}", tok),
                    },
                    None => panic!("for line {}, expected instruction kind", l),
                },
                amount: match tokens.next() {
                    Some(tok) => tok.parse().unwrap(),
                    None => panic!("for line {}, expected operand", l),
                },
            }
        })
        .collect();

    let part1 = instructions
        .iter()
        .fold(State { x: 0, y: 0, aim: 0 }, |mut state, ins| {
            state.applyPart1(ins);
            state
        })
        .value();

    println!("part 1 {}", part1);

    let part2 = instructions
        .iter()
        .fold(State { x: 0, y: 0, aim: 0 }, |mut state, ins| {
            state.applyPart2(ins);
            state
        })
        .value();

    println!("part2 1 {}", part2);

    Ok(())
}
