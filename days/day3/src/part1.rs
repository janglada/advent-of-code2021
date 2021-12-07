extern crate core;

use core::utils::donwload_puzzle;
use itertools::Itertools;
use std::io::Error;

struct Bits {
    bits: Vec<i32>,
}

impl Bits {
    pub fn visit(&mut self, indx: usize, b: i32) {
        // println!("indx = {}", indx);
        // println!("b = {}", b);
        // println!("bits = {:?}", self.bits);
        // println!("self.bits[]  = {}", self.bits[indx]);
        self.bits[indx] = self.bits[indx] + (b * 2 - 1)
    }
    pub fn default(len: usize) -> Bits {
        Bits { bits: vec![0; len] }
    }

    pub fn value(&self) -> String {
        self.bits
            .iter()
            .map(|v| if *v > 0 { "1" } else { "0" })
            .join("")
    }
}

pub async fn solve_part1() {
    let input: String = donwload_puzzle(3, 1).await.unwrap();

    let the_bits = input.lines().fold(Bits::default(12), |mut bits, value| {
        value
            .chars()
            .map(|s| s.to_digit(10).unwrap() as i32)
            .enumerate()
            .for_each(|(i, b)| {
                bits.visit(i, b);
            });
        // println!("{:?}", iter);
        // bits.visit(iter.next().unwrap().to_string().parse().unwrap());

        bits
    });
    let gamma_str = the_bits.value();
    let epsilon = gamma_str
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => {
                panic!()
            }
        })
        .join("");

    let gammaInt = isize::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilonInt = isize::from_str_radix(epsilon.as_str(), 2).unwrap();
    println!(" {}", gammaInt);
    println!(" {}", epsilonInt);
    println!(" {}", gammaInt * epsilonInt);
    println!(" {:?}", the_bits.bits);
}
