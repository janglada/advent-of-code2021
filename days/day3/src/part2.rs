extern crate core;

use core::utils::donwload_puzzle;
use std::slice::Iter;

pub async fn solve_part2() {
    let input: String = donwload_puzzle(3).await.unwrap();
    __solve_part2(input, 12);
}

struct Res {
    mask: isize,
    remaining: usize,
}
///
///
///
///
fn __solve_part2(input: String, n: usize) {
    let the_bits: Vec<isize> = input
        .lines()
        .map(|line| isize::from_str_radix(line, 2).unwrap())
        .collect();

    let mut filter_option: Option<isize> = None;
    let mut o2rating = 0;
    for i in (0..n).rev() {
        // println!("i = {}", i);
        let res = extract_bit_at(the_bits.iter(), i, filter_option, true);
        o2rating = res.mask;
        filter_option = Some(o2rating);
        // println!("mask {:05b} {}", res, res)
        if res.remaining == 1 {
            break;
        }
    }
    let mut co2rating = 0;
    filter_option = None;
    for i in (0..n).rev() {
        // println!("i = {}", i);
        let res = extract_bit_at(the_bits.iter(), i, filter_option, false);
        co2rating = res.mask;
        filter_option = Some(co2rating);
        if res.remaining == 1 {
            break;
        }
        // println!("mask {:05b} {}", res, res)
    }

    println!(" co2rating {} ({:05b})", co2rating, co2rating);
    println!(" o2rating {} ({:05b})", o2rating, o2rating);
    println!(" {} ", co2rating * o2rating)
}

fn mask_nfirst(min: usize, max: usize) -> isize {
    let mut mask = 0;
    for j in min..max {
        mask = mask | 1 << j;
    }
    mask
}

fn extract_bit_at(the_bits: Iter<isize>, index: usize, p: Option<isize>, o2_rating: bool) -> Res {
    // let m1 = mask_nfirst(index, ns);

    let rows: Vec<&isize> = the_bits
        .filter(|n| match p {
            None => true,
            Some(b) => {
                // println!(
                //     "{} {:05b}  {:05b}, ({})",
                //     index,
                //     n,
                //     *n >> (index + 1),
                //     (*n >> (index + 1)) == b
                // );

                (*n >> (index + 1)) == b
            }
        })
        .collect();
    let sum: isize = rows.iter().map(|n| 2 * ((**n >> index) & 1) - 1).sum();
    let mut next_mask = p.unwrap_or(0);

    if rows.len() == 1 {
        // println!("{:?}", rows);
        next_mask = **rows.get(0).unwrap();
    } else {
        if o2_rating {
            if sum >= 0 {
                next_mask = next_mask << 1 | 1;
            } else {
                // next_mask = next_mask | next_mask >> 1;
                next_mask = next_mask << 1;
            }
        } else {
            if sum >= 0 {
                next_mask = next_mask << 1;
            } else {
                // next_mask = next_mask | next_mask >> 1;
                next_mask = next_mask << 1 | 1;
            }
        }
    }
    // println!("sum {}, next mask {:b}", sum, next_mask);
    Res {
        remaining: rows.len(),
        mask: next_mask,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn masks1() {
        let s = "11101";
        let n = isize::from_str_radix(s, 2).unwrap();

        for i in (0..5).rev() {
            println!(" N = {} {}", ((n >> i) & 1), i);
        }
    }

    #[test]
    fn masks() {
        let s = "10101";
        let num = isize::from_str_radix(s, 2).unwrap();
        let m = mask_nfirst(2, 5); //1 << 4 | 1 << 3 | 1 << 2;
        let res = m & num;
        println!("{} {:05b} {:05b}", s, m, res);

        let mut r = 1;
        println!("{:05b}", r);
        r = r << 1; // 10
        println!("{:05b}", r);
        r = r << 1 | 1; // 101
        println!("{:05b}", r);
        r = r << 1; // 1010
        println!("{:05b}", r);
        r = r << 1 | 1; // 101
        println!("{:05b}", r); // 10101
    }

    #[test]
    fn it_works() {
        let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

        __solve_part2(input.to_string(), 5);
    }
}
