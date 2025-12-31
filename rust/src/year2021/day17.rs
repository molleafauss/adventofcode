// https://adventofcode.com/2021/day/17

use std::str::FromStr;
use log::{debug, info};
use once_cell::sync::Lazy;
use regex::Regex;
use adventofcode::Solver;

pub struct Solution {
    xrange: (i32, i32),
    yrange: (i32, i32),
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            xrange: (0, 0),
            yrange: (0, 0),
        }
    }

    fn brute_solve(&self) -> u32 {
        // very brute force, but we're talking very little numbers...

        // can't go faster than this or it won't even reach within the 1st step
        let max_vx = self.xrange.1;
        // won't be going slower than this or it will never reach the border of the trench
        let min_vx = (((-1.0 + (1.0 + 8.0 * self.xrange.0 as f64).sqrt()) / 2.0).ceil()) as i32;
        debug!("Min Vx to reach {}: {} => {}", self.xrange.0, min_vx, min_vx * (min_vx + 1) / 2);
        // can't go higher than this (positive) because when it will land back at y = 0 should at
        // least be within range in one step
        let max_vy = self.yrange.0.abs() - 1;

        // now let's keep iterating
        let mut valid_vectors = 0;
        for vx in min_vx..max_vx + 1 {
            let mut vy = max_vy;
            loop {
                let mut dx = vx;
                let mut dy = vy;
                let mut t = 0;
                let mut x = 0;
                let mut y = 0;
                loop {
                    if self.xrange.0 <= x && x <= self.xrange.1
                        && self.yrange.0 <= y && y <= self.yrange.1 {
                        valid_vectors += 1;
                        debug!("({valid_vectors}) ({vx},{vy}) will reach trench at {t} => {x}, {y}");
                        break;
                    }
                    x += dx;
                    y += dy;
                    t += 1;
                    // debug!("... {x}, {y} @ {t}");
                    if dx > 0 {
                        dx -= 1;
                    }
                    dy -= 1;

                    // too far or too below
                    if x > self.xrange.1 || y < self.yrange.0 {
                        // debug!("({vx}, {vy}) @ {t} => unreachable: {x}, {y}");
                        break;
                    }
                }
                vy -= 1;
                if vy < self.yrange.0 {
                    break;
                }
            }
        }
        valid_vectors
    }
}

const RE_AREA: Lazy<Regex> = Lazy::new(|| Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap());

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if let Some(captures) = RE_AREA.captures(line) {
            self.xrange = (
                i32::from_str(&captures[1]).unwrap(),
                i32::from_str(&captures[2]).unwrap(),
            );
            self.yrange = (
                i32::from_str(&captures[3]).unwrap(),
                i32::from_str(&captures[4]).unwrap(),
            );
        } else if !line.is_empty() {
            panic!("Line doesn't match format!");
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("Checking trench: x = {:?}, y = {:?}", self.xrange, self.yrange);
        /* Some reminder: the distance reachable by a velocity V, (in any direction), considering it
         * will reduce by 1 every "integer" turn, is the "known" formula V(V+1)/2.
         * Knowing the distance, we can calculate a minimum V that satisfies the equation, we'll have
         * to round up as the result will be likely be fractional
         */

        // irrespective of the X position, as long as the Vx can land within the range at the appointed
        // time (there's plenty), the Vy will gradually reduce, then peak at 0, and then finally start
        // getting down, reaching -(Vy+1) when it crosses 0 going down.
        // This makes the best Vy = 1 less than the bottom of the trench.
        let max_vy = self.yrange.0.abs() - 1;
        let max_y = max_vy * (max_vy + 1) / 2;
        info!("[1] Top Vy = {max_vy}, max y reached {max_y}");

        let reachable = self.brute_solve();
        info!("[2] velocity vectors will reach trench: {}", reachable);

        Some((max_y.to_string(), reachable.to_string()))
    }
}
