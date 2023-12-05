// https://adventofcode.com/2023/day/5

use std::collections::HashMap;
use std::str::FromStr;
use log::{debug, info};
use adventofcode::Solver;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_MAP: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\S+)-to-(\S+) map:").unwrap());

pub struct Solution {
    seeds: Vec<u64>,
    current_map: String,
    maps: HashMap<String, Mapper>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            seeds: Vec::new(),
            maps: HashMap::new(),
            current_map: String::new(),
        }
    }

    fn location_point(&self, val: &u64) -> u64 {
        let mut category = String::from("seed");
        let mut num = *val;
        while category != "location" {
            (num, category) = self.maps.get(&category).unwrap().adjust(num);
        }
        num
    }

    fn location_range(&self, input: (u64, u64)) -> u64 {
        let mut category = String::from("seed");
        let mut ranges = vec![input];
        while category != "location" {
            // for all ranges, calculate where they fit based on the remapping ranges defined
            (ranges, category) = self.maps.get(&category).unwrap().range_remap(ranges);
        }
        // find the lowest range
        ranges.sort_by_key(|range| range.0);
        ranges.first().unwrap().0
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        if line.starts_with("seeds:") {
            self.seeds = line[7..].split(" ").map(|val| u64::from_str(val).unwrap()).collect();
            return;
        }

        let captures = RE_MAP.captures(line);
        if captures.is_some() {
            let fields = captures.unwrap();
            let from = String::from(&fields[1]);
            let to = String::from(&fields[2]);
            self.current_map = from.clone();
            self.maps.insert(from.clone(), Mapper { from, to, ranges: Vec::new() });
            return;
        }

        self.maps.get_mut(&self.current_map).unwrap().ranges.push(Range::from(line));
    }

    fn solve(&mut self) -> Option<(String, String)> {
        debug!("Seeds: {}, mappings: {}", self.seeds.len(), self.maps.len());

        let part1 = self.seeds.iter()
            .map(|val| self.location_point(val))
            .min()
            .unwrap();
        info!("[1] Lowest location number found: {}", part1);

        // part 2 is fun
        let part2 = (0..self.seeds.len() / 2)
            // pair seeds and calculate full ranges
            .map(|idx| (self.seeds[idx * 2], self.seeds[idx * 2] + self.seeds[idx * 2 + 1]))
            // find minimum on the range
            .map(|val| self.location_range(val))
            .min()
            .unwrap();
        info!("[2] Lowest location number found: {}", part2);

        Some((part1.to_string(), part2.to_string()))
    }
}

struct Mapper {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Mapper {
    pub fn adjust(&self, val: u64) -> (u64, String) {
        let next = self.ranges.iter()
            .find(|range| range.source.0 <= val && val < range.source.1)
            .map(|range| range.dest.0 + val - range.source.0)
            .unwrap_or(val);
        (next, self.to.clone())
    }

    fn range_remap(&self, mut ranges: Vec<(u64, u64)>) -> (Vec<(u64, u64)>, String) {
        let mut new_ranges = Vec::new();

        while !ranges.is_empty() {
            // remap the range or what fits in a range, re-add the remainder (if any) for further remapping
            let input = ranges.pop().unwrap();
            let (mapped, remainder) = self.ranges.iter()
                .find(|range| range.source.0 <= input.0 && input.0 < range.source.1)
                .map(|range| {
                    if input.1 >= range.source.1 {
                        // doesn't fit into range -> split
                        (
                            (range.dest.0 + input.0 - range.source.0, range.dest.1),
                            Some((range.source.1, input.1))
                        )
                    } else {
                        // fit into range -> just remap, remainder is None
                        ((range.dest.0 + input.0 - range.source.0, range.dest.0 + input.1 - range.source.0), None)
                    }
                })
                // doesn't fit in any mapping range -> return unchanged, remainder is None
                .unwrap_or((input, None));
            new_ranges.push(mapped);
            if remainder.is_some() {
                ranges.push(remainder.unwrap());
            }
        }

        (new_ranges, self.to.clone())
    }
}

struct Range {
    source: (u64, u64),
    dest: (u64, u64),
}

impl Range {
    fn from(line: &str) -> Range {
        let mut vals = line.split(" ").map(|val| u64::from_str(val).unwrap());
        let to = vals.next().unwrap();
        let from = vals.next().unwrap();
        let delta = vals.next().unwrap();

        Range {
            source: (from, from + delta),
            dest: (to, to + delta),
        }
    }
}