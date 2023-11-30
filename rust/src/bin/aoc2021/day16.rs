// https://adventofcode.com/2021/day/16

use std::collections::HashMap;
use log::{debug, info};
use once_cell::sync::Lazy;
use adventofcode::Solver;
use adventofcode::utils::{ONE, ZERO};

pub struct Solution {
    packets: Vec<Packet>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            packets: Vec::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        debug!("Parsing data: {line}");
        let mut stream = BitStream::hex_decode(line);
        self.packets.push(Packet::parse(&mut stream));
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let mut total_version = 0;
        let mut result = 0;
        for pkt in &self.packets {
            total_version += pkt.total_version;
            let val = pkt.solve();
            debug!("Solved packet: {val}");
            result += val;
        }
        info!("[1] Total version of packet: {}", total_version);
        info!("[2] Solution: {}", result);
        Some((total_version.to_string(), result.to_string()))
    }
}

const HEX_MAP: Lazy<HashMap<u8, [u8; 4]>> = Lazy::new(|| HashMap::from([
        ('0' as u8, [ZERO, ZERO, ZERO, ZERO]),
        ('1' as u8, [ZERO, ZERO, ZERO, ONE]),
        ('2' as u8, [ZERO, ZERO, ONE, ZERO]),
        ('3' as u8, [ZERO, ZERO, ONE, ONE]),
        ('4' as u8, [ZERO, ONE, ZERO, ZERO]),
        ('5' as u8, [ZERO, ONE, ZERO, ONE]),
        ('6' as u8, [ZERO, ONE, ONE, ZERO]),
        ('7' as u8, [ZERO, ONE, ONE, ONE]),
        ('8' as u8, [ONE, ZERO, ZERO, ZERO]),
        ('9' as u8, [ONE, ZERO, ZERO, ONE]),
        ('A' as u8, [ONE, ZERO, ONE, ZERO]),
        ('B' as u8, [ONE, ZERO, ONE, ONE]),
        ('C' as u8, [ONE, ONE, ZERO, ZERO]),
        ('D' as u8, [ONE, ONE, ZERO, ONE]),
        ('E' as u8, [ONE, ONE, ONE, ZERO]),
        ('F' as u8, [ONE, ONE, ONE, ONE]),
    ]));

struct Packet {
    total_version: u32,
    version: u32,
    type_id: u32,
    literal: u64,
    packets: Vec<Packet>
}

impl Packet {
    fn parse(stream: &mut BitStream) -> Packet {
        let mut packet = Packet {
            total_version: 0,
            version: 0,
            type_id: 0,
            literal: 0,
            packets: Vec::new(),
        };
        packet.read(stream);
        packet
    }

    fn read(&mut self, stream: &mut BitStream) {
        self.version = stream.parse_int(3) as u32;
        self.total_version += self.version;
        self.type_id = stream.parse_int(3) as u32;
        if self.type_id == 4 {
            // literal value
            self.parse_value(stream)
        } else {
            // parse operation
            self.parse_operation(stream);
        }
    }

    fn parse_value(&mut self, stream: &mut BitStream) {
        loop {
            let flag = stream.parse_flag();
            let val = stream.parse_int(4);
            self.literal <<= 4;
            self.literal += val;
            if !flag {
                break;
            }
        }
        debug!("Found literal value - version: {} / {} ", self.version, self.literal);
    }

    fn parse_operation(&mut self, stream: &mut BitStream) {
        let sub_number = stream.parse_flag();
        if sub_number {
            let mut num_packets = stream.parse_int(11);
            debug!("Found operator packet - version: {}, type id {} - contains {} packets", self.version, self.type_id, num_packets);
            while num_packets > 0 {
                let packet = Packet::parse(stream);
                self.total_version += packet.total_version;
                self.packets.push(packet);
                num_packets -= 1;
            }
        } else {
            // read byte length
            let bytes = stream.parse_int(15) as usize;
            debug!("Found operator packet - version: {}, type id {} - contains {} bytes", self.version, self.type_id, bytes);
            let start = stream.pos;
            while stream.pos - start < bytes {
                let packet = Packet::parse(stream);
                self.total_version += packet.total_version;
                self.packets.push(packet);
            }
            assert_eq!(stream.pos - start, bytes, "Read more than {bytes} from {start}?");
        }
    }

    fn solve(&self) -> u64 {
        match self.type_id {
            // sum of child packets
            0 => self.packets.iter().map(|p| p.solve()).sum(),
            // product of child packets
            1 => self.packets.iter().map(|p| p.solve()).reduce(|acc, e| acc * e).unwrap(),
            // min of child packets
            2 => self.packets.iter().map(|p| p.solve()).min().unwrap(),
            // max of child packets
            3 => self.packets.iter().map(|p| p.solve()).max().unwrap(),
            // literal value
            4 => self.literal,
            // greater than - expect 2 packets
            5 => {
                assert_eq!(self.packets.len(), 2, "More than 2 packets for type id 5 (greather than)");
                if self.packets[0].solve() > self.packets[1].solve() { 1 } else { 0 }
            },
            // less than - expect 2 packets
            6 => {
                assert_eq!(self.packets.len(), 2, "More than 2 packets for type id 6 (less than)");
                if self.packets[0].solve() < self.packets[1].solve() { 1 } else { 0 }
            },
            // equal - expect 2 packets
            7 => {
                assert_eq!(self.packets.len(), 2, "More than 2 packets for type id 7 (equal)");
                if self.packets[0].solve() == self.packets[1].solve() { 1 } else { 0 }
            },
            _ => panic!("Unsupported operation for type id: {}", self.type_id),
        }
    }
}

struct BitStream {
    pos: usize,
    data: Vec<u8>,
}

impl BitStream {
    fn hex_decode(text: &str) -> BitStream {
        BitStream {
            pos: 0,
            data: text.bytes().map(|ch| HEX_MAP.get(&ch).unwrap().clone()).flatten().collect(),
        }
    }

    fn parse_int(&mut self, mut count: u32) -> u64 {
        let mut val = 0;
        while count > 0 {
            count -= 1;
            val *= 2;
            val += if self.data[self.pos] == ONE { 1 } else { 0 };
            self.pos += 1;
        }
        val
    }

    fn parse_flag(&mut self) -> bool {
        let flag = self.data[self.pos] == ONE;
        self.pos += 1;
        flag
    }
}