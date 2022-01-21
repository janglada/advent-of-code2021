extern crate core;

use core::utils::donwload_puzzle;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(16).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{parse, Packet, PacketParser, PacketType};
    use std::borrow::Borrow;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn test_string_parser() {
        let bits = parse("D2FE28".to_string());
        println!("{}", bits);

        assert_eq!(bits, "110100101111111000101000");

        let mut parser = PacketParser::new(bits.borrow());
        let next_bits = parser.read(3);
        assert_eq!(next_bits.unwrap(), "110");
        let next_bits = parser.read(3);
        assert_eq!(next_bits.unwrap(), "100");
    }

    #[test]
    fn test_packet_parser_I() {
        let bits = parse("D2FE28".to_string());
        let mut packet_parser = PacketParser::new(bits.as_str());
        let packet = Packet::from(&mut packet_parser);

        assert_eq!(6, packet.version);
        match packet.packet_type {
            PacketType::Type4(c) => {
                assert_eq!(2021, c);
            }
            PacketType::Operator(_) => {
                panic!("Wrong operator");
            }
        }
    }

    #[test]
    fn test_packet_parser_II() {
        let bits = parse("38006F45291200".to_string());
        // let mut packet_parser = PacketParser::new(bits.as_str());
        let packet = Packet::read_packets(bits.as_str());

        // assert_eq!(6, packet.version);
        // match packet.packet_type {
        //     PacketType::Type4(c) => {
        //         assert_eq!(2021, c);
        //     }
        //     PacketType::Operator(_) => {
        //         panic!("Wrong operator");
        //     }
        // }
    }
}
