use std::thread::sleep;

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
    Type4(u32),
    Operator(Vec<Packet>),
}

enum OperatorType {}

#[derive(PartialEq)]
pub struct Packet {
    pub version: u32,
    pub packet_type: PacketType,
}

impl<'a> Packet {
    pub fn read_packets(s: &'a str) -> Vec<Packet> {
        let mut packets = Vec::new();
        let mut packet_parser = PacketParser::new(s);

        while let Some(i) = packet_parser.peek(1) {
            packets.push(Packet::from(&mut packet_parser));
        }
        packets
    }

    pub fn from(packet_parser: &mut PacketParser) -> Packet {
        let version = u32::from_str_radix(packet_parser.read(3).unwrap(), 2).unwrap();
        let type_id = u32::from_str_radix(packet_parser.read(3).unwrap(), 2).unwrap();

        let packet_type = match type_id {
            4 => {
                let mut is_last = false;
                let mut s = String::new();
                while !is_last {
                    is_last = packet_parser.read(1).unwrap().parse::<u32>().ok().unwrap() == 0;
                    s.push_str(packet_parser.read(4).unwrap());
                }

                while let Some(i) = packet_parser.peek(1) {
                    println!("WHILE {}", i);
                    if i != "0" {
                        break;
                    }
                }

                PacketType::Type4(u32::from_str_radix(s.as_str(), 2).unwrap())
            }
            operator => {
                let length_id = packet_parser.read(1).unwrap().parse::<u32>().ok().unwrap();

                let packets = match length_id {
                    0 => {
                        let sub_packet_bits_len =
                            usize::from_str_radix(packet_parser.read(15).unwrap(), 2).unwrap();
                        let sub_packet_bits = packet_parser.read(sub_packet_bits_len);
                        Packet::read_packets(sub_packet_bits.unwrap())
                    }
                    1 => {
                        let sub_packet_bits_len =
                            u32::from_str_radix(packet_parser.read(11).unwrap(), 2).unwrap();

                        Packet::read_packets(packet_parser.read_all().unwrap())
                    }
                    _ => {
                        panic!("Unexpected")
                    }
                };

                PacketType::Operator(packets)
            }
        };

        Packet {
            version,
            packet_type,
        }
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
        if self.chars.len() <= self.position + n {
            None
        } else {
            let s = &self.chars[self.position..self.position + n];
            self.position = self.position + n;
            Some(s)
        }
    }

    pub fn read_all(&mut self) -> Option<&str> {
        if self.chars.len() <= self.position {
            None
        } else {
            let s = &self.chars[self.position..self.chars.len()];
            self.position = self.chars.len();
            Some(s)
        }
    }

    pub fn peek(&self, n: usize) -> Option<&str> {
        if self.chars.len() <= self.position + n {
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
