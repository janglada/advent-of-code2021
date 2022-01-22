extern crate core;

use crate::shared::{parse, Packet};
use core::utils::donwload_puzzle;
use std::slice::Iter;

pub async fn solve_part2() {
    let input: String = donwload_puzzle(16).await.unwrap();
    let bits = parse(input);
    // let mut packet_parser = PacketParser::new(bits.as_str());
    let packet = Packet::read_packets(bits.as_str(), true);
    println!("Part 2 score = {}", Packet::score_all(&packet));
}
