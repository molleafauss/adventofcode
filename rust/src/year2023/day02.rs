// https://adventofcode.com/2023/day/2
// learned about trim_end_matches - interesting way to do some matching without regexp

use std::str::FromStr;

use log::{debug, info};

use adventofcode::Solver;

pub struct Solution {
    part1: u32,
    part2: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            part1: 0,
            part2: 0,
        }
    }
}

// cubes for first part (red, green, blue)
const CUBES: [u32; 3] = [12, 13, 14];
const SEPS: [char; 2] = [',', ';'];

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        let mut parts = line.split(" ");
        if parts.next().unwrap() != "Game" {
            panic!("Not a valid line? {}", line);
        }
        let id_str = parts.next().unwrap();
        // exclude last ':'
        let game_id = u32::from_str(&id_str[..id_str.len() - 1]).unwrap();
        let mut game_valid = true;
        let mut min_cubes = [0, 0, 0];

        loop {
            let next_item = parts.next();
            if next_item.is_none() {
                break;
            }
            let num_cubes = u32::from_str(next_item.unwrap()).unwrap();
            let mut cube_type = parts.next().unwrap();
            if cube_type.rfind(SEPS).is_some() {
                cube_type = cube_type.trim_end_matches(SEPS);
            }
            if cube_type == "red" {
                game_valid &= num_cubes <= CUBES[0];
                min_cubes[0] = min_cubes[0].max(num_cubes);
            } else if cube_type == "green" {
                game_valid &= num_cubes <= CUBES[1];
                min_cubes[1] = min_cubes[1].max(num_cubes);
            } else if cube_type == "blue" {
                game_valid &= num_cubes <= CUBES[2];
                min_cubes[2] = min_cubes[2].max(num_cubes);
            } else {
                panic!("Invalid colour: {}", cube_type);
            }
        }
        let power = min_cubes[0] * min_cubes[1] * min_cubes[2];
        debug!("Game {} - possible? {} - min cubes {:?}, power {}", game_id, game_valid, min_cubes, power);
        self.part1 += if game_valid { game_id } else { 0 };
        self.part2 += power;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Possible game ids sum: {}", self.part1);
        info!("[2] Min cubes power: {}", self.part2);

        Some((self.part1.to_string(), self.part2.to_string()))
    }
}
