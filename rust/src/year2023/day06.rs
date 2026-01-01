// https://adventofcode.com/2023/day/6

use std::str::FromStr;
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    times: Vec<u32>,
    distances: Vec<u32>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            times: Vec::new(),
            distances: Vec::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.starts_with("Time:") {
            self.times = line[5..].split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| u32::from_str(v).unwrap())
                .collect();
        } else if line.starts_with("Distance:") {
            self.distances = line[9..].split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| u32::from_str(v).unwrap())
                .collect();
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let races = self.times.len();
        assert_eq!(races, self.distances.len());
        info!("Found {} times and distances", races);

        let mut part1 = 0;
        for i in 0..races {
            let t = self.times[i] as f64;
            let d = self.distances[i] as f64;

            // some Dream Theater please?
            let record_beaten = distance_over_time(d, t);
            if record_beaten == 0 {
                continue;
            }

            if part1 == 0 {
                part1 = record_beaten;
            } else {
                part1 *= record_beaten;
            };
        }
        info!("[1] Records : {part1}");

        let big_t = collapse(&self.times);
        let big_d = collapse(&self.distances);
        let part2 = distance_over_time(big_d, big_t);
        info!("[2] T {big_t}, D {big_d} => Records: {part2}");

        Some((part1.to_string(), part2.to_string()))
    }
}

fn collapse(nums: &Vec<u32>) -> f64 {
    let mut text = String::new();
    nums.iter().for_each(|num| text.push_str(&num.to_string()));
    f64::from_str(&text).unwrap()
}

fn distance_over_time(d: f64, t: f64) -> u32 {
    // second degree (dis)equation: given t and d, we'll move by x * (t - x); and we need
    // to find where this is >= d (x is the time we press the button)
    // so equation is -x^2 + tx - d >= 0
    // find zeroes (they need to be both > 0) and find range inside (round up and down to
    // nearest integers)
    let delta = t.powf(2_f64) - 4_f64 * d;
    if delta < 0_f64 {
        info!("No zeroes for T = {t} and D = {d}");
        return 0;
    }
    let mut min = ((t - delta.sqrt()) / 2.0).ceil();
    let mut max = ((t + delta.sqrt()) / 2.0).floor();
    if min == max {
        info!("Only one zero for T = {t} and D = {d}");
        return 0;
    }
    if min * (t - min) == d {
        min += 1_f64;
    }
    if max * (t - max) == d {
        max -= 1_f64;
    }
    let record_beaten = max as u32 - min as u32 + 1;
    debug!("time {t}, distance {d}: -x^2 + {t}x - {d} = ({min}, {max}) => {record_beaten} possible records");

    record_beaten
}
