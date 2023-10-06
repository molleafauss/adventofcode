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

    fn trench_reachable(&self, vx: i32, vy: i32, t: i32, open_interval: bool) -> Option<bool> {
        // x is within range - we know this; y is "reversed" (all negative)
        let mut end_y;
        if t <= vy + 1 {
            // still on the "ascending" part (and implicitly t > 0)
            end_y = t * (2 * vy - t + 1) / 2;
        } else {
            // find the peak and then count down
            end_y = (vy * (vy + 1) / 2) - (t - vy) * (t - vy - 1) / 2
        };
        let reachable = self.yrange.0 <= end_y && end_y <= self.yrange.1;
        if reachable {
            debug!("Reachability found for ({vx}, {vy}) within {t} (open interval: {open_interval}) => {end_y}");
            return Some(true);
        }

        // if open ended, keep iterating until pos is > lower y
        let mut nextt = t + 1;
        let mut nextv = vy - t;
        while open_interval && self.yrange.0 <= end_y {
            nextt += 1;
            nextv -= 1;
            end_y += nextv;
            if end_y <= self.yrange.1 {
                debug!("Reachability found for ({vx}, {vy}) within {nextt} [{t} => ...] => {end_y} (at vy {nextv})");
                return Some(true)
            }
        }

        // find first t that exceed bottom range, and verify if we can never be inside the trench (from below)
        let b = (2.0 * vy as f64 + 1.0) / 2.0;
        let min_t = (b + (b.powi(2) - 2.0 * (self.yrange.0 - 1) as f64).sqrt()).floor() as i32;
        if vy < 0 && min_t <= t {
            // stop here - from now on we will never be able to get within range
            return None
        }

        Some(false)
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
        let max_vy = (self.yrange.0 + 1).abs();
        let max_y = max_vy * (max_vy + 1) / 2;
        info!("[1] Top Vy = {max_vy}, max y reached {max_y}");

        // now try to find all the valid vx, vy that can hit the target.
        // on the X axis we find the minimum Vx we must have to reach the closes border of the trench
        let min_vx = (((-1.0 + (1.0 + 8.0 * self.xrange.0 as f64).sqrt()) / 2.0).ceil()) as i32;
        let min_dx = min_vx * (min_vx + 1) / 2;
        debug!("Min Vx to reach {}: {} => {}", self.xrange.0, min_vx, min_dx);

        // now loop from max_vy down and see how many hit.
        let mut valid_initials = 0;
        for vx in (min_vx..self.xrange.1 + 1) {
            let xtrange = find_xtime_range(vx, self.xrange);
            if xtrange.is_empty() {
                continue;
            }
            let mut stop_descending = false;
            let mut vy = max_vy;
            // keep iterating until false or none
            while !stop_descending {
                for (t, open_interval) in &xtrange {
                    let result = self.trench_reachable(vx, vy, *t, *open_interval);
                    if let Some(true) = result {
                        valid_initials += 1;
                        break
                    } else if result.is_none() {
                        stop_descending = true;
                        break;
                    }
                }
                vy -= 1;
            }
        }
        info!("[2] Found valid vx/vy pairs: {valid_initials}");

        Some((max_y.to_string(), valid_initials.to_string()))
    }
}

/// finds the time range where the velocity v - decelerating to 0 - will put an object within range
fn find_xtime_range(v: i32, range: (i32, i32)) -> Vec<(i32, bool)> {
    let mut times = Vec::new();

    // find min t that can reach the range (find the T that satisfies v + ... + v - t + 1 >= range.0
    let b = (2.0 * v as f64 + 1.0) / 2.0;
    let min_t = (b - (b.powi(2) - 2.0 * range.0 as f64).sqrt()).ceil() as i32;
    let t_end = (b - (b.powi(2) - 2.0 * range.1 as f64).sqrt()).floor();
    let mut stop_inside = false;

    let max_t = if t_end.is_nan() {
        stop_inside = true;
        v as i32
    } else {
        t_end as i32
    };
    if max_t < min_t {
        // can't reach inside
        debug!("No valid times for {v} in {:?} [{min_t}/{max_t} - {t_end}]", range);
        return times;
    }
    debug!("Found t range for {v}: {min_t}/{max_t} [{t_end}]");
    for t in min_t..max_t + 1 {
        times.push((t, false));
    }
    times.last_mut().unwrap().1 = stop_inside;

    times
}
