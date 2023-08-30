// What did I learn?
// arranging mutable/immutable borrow to not overlap. Destructuring structs.
// resulting time: 3.5 sec vs 166 (\o/ expected)

use std::cmp::max;
use std::collections::HashSet;
use std::str::FromStr;
use log::{debug, info};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::grid::GridPos;
use crate::Solver;

pub(crate) struct Solution {
    sensors: Vec<Sensor>,
    y: i64,
    area: i64,
}

static RE_SENSOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap());

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            sensors: Vec::new(),
            y: 2000000,
            area: 4000000,
        }
    }

    fn check_line(&self, y: i64) -> (Vec<Segment>, HashSet<i64>) {
        let mut segments = Vec::new();
        let mut beacons = HashSet::new();
        // find all invalid segment in line y
        for sensor in &self.sensors {
            if y < sensor.position.row - sensor.distance || sensor.position.row + sensor.distance < y {
                // not in range
                continue;
            }
            let delta = sensor.distance - (y - sensor.position.row).abs();
            merge(&mut segments, sensor.position.col - delta, sensor.position.col + delta);
            if sensor.beacon.row == y {
                beacons.insert(sensor.beacon.col);
            }
        }
        (segments, beacons)
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if line.starts_with("part 1: ") {
            self.y = i64::from_str(&line[8..]).unwrap();
            return;
        }
        if line.starts_with("part 2: ") {
            self.area = i64::from_str(&line[8..]).unwrap();
            return;
        }
        if let Some(captures) = RE_SENSOR.captures(line) {
            let position = GridPos::of(
                i64::from_str(&captures[1]).unwrap(),
                i64::from_str(&captures[2]).unwrap(),
            );
            let beacon = GridPos::of(
                i64::from_str(&captures[3]).unwrap(),
                i64::from_str(&captures[4]).unwrap(),
            );
            let distance = m_distance(&position, &beacon);
            self.sensors.push(Sensor {
                _id: self.sensors.len() + 1,
                position,
                beacon,
                distance,
            });
        } else {
            panic!("Unknown line: {line}");
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        debug!("We have {} sensors", self.sensors.len());
        debug!("part 1 - finding invalid beacon positions at line {}", self.y);
        let (segments, beacons) = self.check_line(self.y);
        let segment_size = segment_length(&segments) - beacons.len() as i64;
        info!("[1] invalid set contains {segment_size} elements");

        debug!("Finding possible real beacon positions in area 0-{}", self.area);
        let mut frequency = 0;
        for y in 0..self.area + 1 {
            if (y % 100000) == 0 {
                println!("Checking line {y}/{}", self.area);
            }
            let (mut segments, beacons) = self.check_line(y);
            for b in beacons {
                merge(&mut segments, b, b);
            }
            if segments.len() == 1 && segments[0].start <= 0 && segments[0].end >= self.area {
                continue;
            }
            debug!("Found something at y: {y} - {}?", segments.len());
            let x = segments[0].end + 1;
            assert_eq!(x, segments[1].start - 1);
            frequency = x as i64 * FREQ_MULT + y as i64;
            info!("[2] Found frequency: {frequency}");
            break;
        }
        Some((segment_size.to_string(), frequency.to_string()))
    }
}

const FREQ_MULT: i64 = 4000000;

fn m_distance(from: &GridPos, to: &GridPos) -> i64 {
    (from.col - to.col).abs() + (from.row - to.row).abs()
}

fn segment_length(segments: &Vec<Segment>) -> i64 {
    segments.iter().map(|seg| seg.end - seg.start + 1).sum()
}

fn merge(segments: &mut Vec<Segment>, x_start: i64, x_end: i64) {
    assert!(x_start <= x_end);
    let mut i = 0;
    // insert segment
    while i < segments.len() && segments[i].start < x_start {
        i += 1;
    }
    if i == segments.len() {
        segments.push(Segment{start: x_start, end: x_end});
    } else {
        segments.insert(i, Segment{start: x_start, end: x_end});
    }

    // now merge segments if they overlap
    i = 0;
    while i < segments.len() - 1 {
        let Segment {start: s1_start, end: s1_end} = segments[i + 1];
        let s0 = &mut segments[i];
        if s1_start <= s0.end + 1 {
            s0.end = max(s0.end, s1_end);
            assert!(s0.end >= s0.start);
            segments.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

struct Segment {
    start: i64,
    end: i64,
}

struct Sensor {
    _id: usize,
    position: GridPos,
    beacon: GridPos,
    distance: i64,
}