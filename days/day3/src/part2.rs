extern crate core;

use core::utils::donwload_puzzle;
use std::slice::Iter;

pub async fn solve_part2() {
    let input: String = donwload_puzzle(3, 1).await.unwrap();

    let the_bits: Vec<isize> = input
        .lines()
        .map(|line| isize::from_str_radix(line, 2).unwrap())
        .collect();

    let mut filterOption: Option<isize> = None;
    for i in 1..13 {
        let idx = 12 - i;
        let res = extractBitAt(the_bits.iter(), idx, filterOption);
        filterOption = Some(res);
        println!("{}", res)
    }
    // let sum: i32 = the_bits
    //     .iter()
    //     .map(|n| (n >> 11) as i32)
    //     .map(|n| 2 * n - 1)
    //     .filter()
    //     .sum();
    // if (sum > 0) {
    //     "1"
    // } else {
    //     "0"
    // }
    // println!("{}", sum);

    // the_bits
    //     .iter()
    //     .for_each(|n| println!("{} {} {}", n, format!("{:b}", n), n >> 11));
}

fn extractBitAt(the_bits: Iter<isize>, index: usize, p: Option<isize>) -> isize {
    let mask: isize = 1 << index;
    println!(" MASK {} {:b}", mask, mask);
    let sum: isize = the_bits
        //   .map(|n| (n >> index) as i32)
        .filter(|n| match p {
            None => true,
            Some(b) => b == ((**n) & mask),
        })
        .map(|n| {
            // println!("{} {}", n, 2 * n - 1);
            2 * n - 1
        })
        .sum();
    if sum >= 0 {
        1
    } else {
        0
    }
}
