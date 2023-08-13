// What did I learn?
// arranging mutable/immutable borrow to not overlap. Destructuring structs.

use std::cmp::max;
use std::collections::HashSet;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::grid::GridPos;
use crate::Solver;

pub(crate) struct Solution {
    sensors: Vec<Sensor>,
    y: i32,
    area: i32,
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

    fn check_line(&self, y: i32) -> (Vec<Segment>, HashSet<i32>) {
        let mut segments = Vec::new();
        let mut beacons = HashSet::new();
        // find all invalid segment in line y
        for sensor in &self.sensors {
            if y < sensor.position.y - sensor.distance || sensor.position.y + sensor.distance < y {
                // not in range
                continue;
            }
            let delta = sensor.distance - (y - sensor.position.y).abs();
            merge(&mut segments, sensor.position.x - delta, sensor.position.x + delta);
            if sensor.beacon.y == y {
                beacons.insert(sensor.beacon.x);
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
            self.y = i32::from_str(&line[8..]).unwrap();
            return;
        }
        if line.starts_with("part 2: ") {
            self.area = i32::from_str(&line[8..]).unwrap();
            return;
        }
        if let Some(captures) = RE_SENSOR.captures(line) {
            let position = GridPos::of(
                i32::from_str(&captures[1]).unwrap(),
                i32::from_str(&captures[2]).unwrap(),
            );
            let beacon = GridPos::of(
                i32::from_str(&captures[3]).unwrap(),
                i32::from_str(&captures[4]).unwrap(),
            );
            let distance = m_distance(&position, &beacon);
            self.sensors.push(Sensor {
                id: self.sensors.len() + 1,
                position,
                beacon,
                distance,
            });
        } else {
            panic!("Unknown line: {line}");
        }
    }

    fn solve(&mut self) {
        println!("We have {} sensors", self.sensors.len());
        println!("part 1 - finding invalid beacon positions at line {}", self.y);
        let (segments, beacons) = self.check_line(self.y);
        let segment_size = segment_length(&segments) - beacons.len() as i32;
        println!("[1] invalid set contains {segment_size} elements");

        println!("Finding possible real beacon positions in area 0-{}", self.area);
        for y in 0..self.area + 1 {
            if (y % 100000) == 0 {
                println!("Checking line {y}");
            }
            let (mut segments, beacons) = self.check_line(y);
            for b in beacons {
                merge(&mut segments, b, b);
            }
            if segments.len() == 1 && segments[0].start <= 0 && segments[0].end >= self.area {
                continue;
            }
            println!("Found something at y: {y} - {}?", segments.len());
            let x = segments[0].end + 1;
            assert_eq!(x, segments[1].start - 1);
            let frequency: i64 = x as i64 * FREQ_MULT + y as i64;
            println!("[2] Found frequency: {frequency}");
            break;
        }
    }
}

const FREQ_MULT: i64 = 4000000;

fn m_distance(from: &GridPos, to: &GridPos) -> i32 {
    (from.x - to.x).abs() + (from.y - to.y).abs()
}

fn segment_length(segments: &Vec<Segment>) -> i32 {
    segments.iter().map(|seg| seg.end - seg.start + 1).sum()
}

fn merge(segments: &mut Vec<Segment>, x_start: i32, x_end: i32) {
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
    start: i32,
    end: i32,
}

struct Sensor {
    id: usize,
    position: GridPos,
    beacon: GridPos,
    distance: i32,
}