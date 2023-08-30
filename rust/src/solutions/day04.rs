// What did I learn?
// fun with iterators, how to extract from them and how to check splits don't have extra values.

use std::str::FromStr;
use log::info;
use crate::Solver;

pub(crate) struct Solution {
    full_overlaps: u32,
    partial_overlaps: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            full_overlaps: 0,
            partial_overlaps: 0
        }
    }
}

struct Elf {
    min: u32,
    max: u32,
}

impl Elf {
    fn parse(sections: &str) -> Elf {
        let mut parts = sections.split("-")
            .map(|text| u32::from_str(text).unwrap());
        let elf = Elf { min: parts.next().unwrap(), max: parts.next().unwrap() };
        if parts.next().is_some() {
            panic!("More parts than expected {sections}");
        }
        elf
    }

    fn inside(&self, other: &Elf) -> bool {
        (other.min <= self.min && self.min <= other.max) || (other.min <= self.max && self.max <= other.max)
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let mut parts = line.split(",");
        let elf1 = Elf::parse(parts.next().unwrap());
        let elf2 = Elf::parse(parts.next().unwrap());
        if parts.next().is_some() {
            panic!("More parts than expected {line}");
        }
        // full overlap when both ends of one of the ranges are fully inside the other
        if (elf1.min <= elf2.min && elf1.max >= elf2.max) || (elf1.min >= elf2.min && elf1.max <= elf2.max) {
            self.full_overlaps += 1;
        }
        // partial overlap if either end of each range is inside the other (one check is enough?)
        if elf1.inside(&elf2) || elf2.inside(&elf1) {
            self.partial_overlaps += 1
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Fully overlapping sections: {}", self.full_overlaps);
        info!("[2] Partially overlapping sections: {}", self.partial_overlaps);
        Some((self.full_overlaps.to_string(), self.partial_overlaps.to_string()))
    }
}