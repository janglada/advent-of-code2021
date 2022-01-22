use std::io::Write;

pub fn parse(input: String) -> String {
    input
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Unexpected char"),
        })
        .collect()
}
#[derive(PartialEq)]
pub enum PacketType {
    Type4(u64),
    Operator(Vec<Packet>),
}

enum OperatorType {}

#[derive(PartialEq)]
pub struct Packet {
    pub version: u32,
    pub packet_type: PacketType,
}

impl<'a> Packet {
    pub fn read_packets(s: &'a str, root: bool) -> Vec<Packet> {
        let mut packets = Vec::new();
        let mut packet_parser = PacketParser::new(s);

        while let Some(i) = packet_parser.peek(1) {
            match Packet::from(&mut packet_parser, root) {
                None => {
                    break;
                }
                Some(p) => {
                    packets.push(p);
                }
            }
        }
        packets
    }

    pub fn from(packet_parser: &mut PacketParser, root: bool) -> Option<Packet> {
        match packet_parser.peek(1) {
            None => None,
            Some(_) => {
                println!("READ VERSION AND TYPE");
                let version = packet_parser.read_u32(3);
                let type_id = packet_parser.read_u32(3);

                let packet_type = match type_id {
                    4 => {
                        println!("READ LITERAL PACKET");
                        let mut is_last = false;
                        let mut s = String::new();
                        while !is_last {
                            is_last = packet_parser.read_u32(1) == 0;
                            s.push_str(packet_parser.read(4).unwrap());
                        }

                        println!(
                            "GOT LITERAL PACKET {}",
                            u64::from_str_radix(s.as_str(), 2).unwrap()
                        );
                        PacketType::Type4(u64::from_str_radix(s.as_str(), 2).unwrap())
                    }
                    _ => {
                        println!("READ OPERATOR PACKET");
                        let length_id = packet_parser.read_u32(1);

                        let packets = match length_id {
                            0 => {
                                let sub_packet_bits_len: usize =
                                    packet_parser.read_u32(15).try_into().unwrap();
                                let sub_packet_bits = packet_parser.read(sub_packet_bits_len);
                                println!(
                                    "  READING SUBPACKETS LENGTH TYPE 0. Sub packets bits length {}",
                                    sub_packet_bits_len
                                );
                                Packet::read_packets(sub_packet_bits.unwrap(), false)
                            }
                            1 => {
                                let sub_packet_count = packet_parser.read_u32(11);
                                let mut v = Vec::new();
                                println!(
                                    "  READING SUBPACKETS LENGTH TYPE 1. There are  {} subpackets",
                                    sub_packet_count
                                );
                                for i in 0..sub_packet_count {
                                    match Packet::from(packet_parser, false) {
                                        None => {
                                            break;
                                        }
                                        Some(p) => {
                                            v.push(p);
                                        }
                                    }
                                }
                                v
                            }
                            _ => {
                                panic!("Unexpected")
                            }
                        };

                        PacketType::Operator(packets)
                    }
                };
                if root {
                    while let Some(i) = packet_parser.peek(1) {
                        // println!(" i = {}  {}", i, i.parse::<u32>().ok().unwrap() != 0);
                        if i.parse::<u32>().ok().unwrap() != 0 {
                            break;
                        }
                        println!("SKIPPING 0");
                        packet_parser.read(1);
                    }
                }

                Some(Packet {
                    version,
                    packet_type,
                })
            }
        }
    }

    pub fn score_all(packets: &Vec<Packet>) -> u32 {
        packets.iter().map(|p| p.score()).sum()
    }

    pub fn score(&self) -> u32 {
        let score = match &self.packet_type {
            PacketType::Type4(_) => 0,
            PacketType::Operator(op) => op.iter().map(|p| p.score()).sum(),
        };
        score + self.version
    }
}

pub struct PacketParser<'a> {
    chars: &'a str,
    position: usize,
}

impl<'a> PacketParser<'a> {
    pub fn new(s: &'a str) -> PacketParser {
        PacketParser {
            chars: s,
            position: 0,
        }
    }

    pub fn read(&mut self, n: usize) -> Option<&str> {
        if self.chars.len() < self.position + n {
            panic!(
                "Trying to read beyond chars length chars = {}, pos = {}, n = {}",
                self.chars.len(),
                self.position,
                n
            );
        } else {
            let s = &self.chars[self.position..self.position + n];
            let prev = self.position;
            self.position = self.position + n;
            println!("{}", self.chars);
            println!("{}{}", " ".repeat(prev), "^".repeat(n));
            Some(s)
        }
    }

    pub fn read_u32(&mut self, n: usize) -> u32 {
        u32::from_str_radix(self.read(n).unwrap(), 2).ok().unwrap()
    }

    pub fn read_all(&mut self) -> Option<&str> {
        if self.chars.len() < self.position {
            None
        } else {
            let s = &self.chars[self.position..self.chars.len()];
            self.position = self.chars.len();
            Some(s)
        }
    }

    pub fn peek(&self, n: usize) -> Option<&str> {
        if self.chars.len() < self.position + n {
            None
        } else {
            let s = &self.chars[self.position..self.position + n];
            Some(s)
        }
    }

    pub fn has_more(&self) -> bool {
        self.chars.len() > self.position
    }
}
