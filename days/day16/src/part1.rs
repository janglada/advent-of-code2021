extern crate core;

use crate::shared::{parse, Packet};
use core::utils::donwload_puzzle;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(16).await.unwrap();
    let bits = parse(input);
    // let mut packet_parser = PacketParser::new(bits.as_str());
    let packet = Packet::read_packets(bits.as_str(), true);
    println!("score = {}", Packet::score_all(&packet));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{parse, Packet, PacketParser, PacketType};
    use std::borrow::Borrow;
    use std::cmp;

    fn assert_is_literal_packet(bits: &str, expected: u32) {
        let mut packet_parser = PacketParser::new(bits);

        let packet = Packet::from(&mut packet_parser, true);
        assert_eq!(true, packet.is_some());
        // assert_eq!(expected, packet.as_ref().unwrap().version);
        match packet.as_ref().unwrap().packet_type {
            PacketType::Type4(c) => {
                assert_eq!(expected, c);
            }
            PacketType::Operator(_) => {
                panic!("Wrong operator");
            }
        }
    }

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
    fn test_packet_parser_literals() {
        assert_is_literal_packet(parse("D2FE28".to_string()).as_str(), 2021);
        assert_is_literal_packet("11010001010", 10);
        assert_is_literal_packet("0101001000100100", 20);
        assert_is_literal_packet("01010010001001000000000", 20);
        assert_is_literal_packet("01010000001", 1);
        assert_is_literal_packet("10010000010", 2);
        assert_is_literal_packet("00110000011", 3);
        assert_is_literal_packet("0011000001100000", 3);
    }

    #[test]
    fn test_packet_parser_iii() {
        // let bits = parse("A0016C880162017C3686B18A3D4780".to_string());
        let bits = parse("EE00D40C823060".to_string());
        // let mut packet_parser = PacketParser::new(bits.as_str());
        let packet = Packet::read_packets(bits.as_str(), true);

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

    #[test]
    fn test_packet_parser_iv() {
        let bits = parse("8A004A801A8002F478".to_string());
        // let mut packet_parser = PacketParser::new(bits.as_str());
        let packet = Packet::read_packets(bits.as_str(), true);
        assert_eq!(Packet::score_all(&packet), 16);
    }

    #[test]
    fn test_packet_parser_v() {
        let bits = parse("620080001611562C8802118E34".to_string());
        // let mut packet_parser = PacketParser::new(bits.as_str());
        let packet = Packet::read_packets(bits.as_str(), true);
        assert_eq!(Packet::score_all(&packet), 12);
    }

    #[test]
    fn test_packet_parser_vi() {
        let bits = parse("C0015000016115A2E0802F182340".to_string());
        // let mut packet_parser = PacketParser::new(bits.as_str());
        let packet = Packet::read_packets(bits.as_str(), true);
        assert_eq!(Packet::score_all(&packet), 23);
    }
    #[test]
    fn test_packet_parser_vii() {
        let bits = parse("A0016C880162017C3686B18A3D4780".to_string());
        // let mut packet_parser = PacketParser::new(bits.as_str());
        let packet = Packet::read_packets(bits.as_str(), true);
        assert_eq!(Packet::score_all(&packet), 31);
    }
}
