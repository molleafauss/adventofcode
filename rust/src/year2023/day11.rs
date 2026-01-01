// https://adventofcode.com/2023/day/11

use log::{debug, info};

use adventofcode::grid::GridPos;
use adventofcode::Solver;

pub struct Solution {
    map: Vec<Vec<char>>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            map: Vec::new(),
        }
    }

    fn calculate_distances(&mut self, delta: usize) -> i64 {
        let height = self.map.len();
        debug!("Finding empty rows ... (map height {})", height);
        let mut skip_rows = Vec::new();
        for row in 0..height {
            if self.map[row].iter().all(|ch| *ch == '.') {
                debug!("row {row} is empty - adding {delta}");
                skip_rows.push((row, delta));
            }
        }

        let mut skip_cols = Vec::new();
        let width = self.map[0].len();
        debug!("Checking empty columns ... (map width {})", width);
        for col in 0..width {
            if (0..height).all(|row| self.map[row][col] == '.') {
                debug!("column {col} is empty - adding {delta}");
                skip_cols.push((col, delta));
            }
        }

        let galaxies: Vec<GridPos> = (0..height).flat_map(|r|
            self.map[r].iter().enumerate().filter_map(|(c, ch)| {
                if *ch == '.' {
                    return None;
                }

                let row = r + skip_rows.iter()
                    .filter_map(|(pos, delta)|
                        if *pos < r { Some(delta) } else { None }
                    ).sum::<usize>();

                let col = c + skip_cols.iter()
                    .filter_map(|(pos, delta)|
                        if *pos < c { Some(delta) } else { None }
                    )
                    .sum::<usize>();

                debug!("Found galaxy at ({r}, {c}) =>  ({row}, {col})");
                // gridpos has terms inverted... (not that it _really_ matters
                Some(GridPos::of(row as i64, col as i64))
            }).collect::<Vec<GridPos>>()
        ).collect();
        info!("Found {} galaxies: {:?}", galaxies.len(), galaxies);

        let mut all_distances = 0;
        for i in 0..galaxies.len() {
            let start = &galaxies[i];
            all_distances += galaxies[i+1..].iter().map(|other| {
                let dist = start.distance(other);
                dist.0.abs() + dist.1.abs()
            }).sum::<i64>();
        }
        all_distances
    }
}

const PART1_DELTA: usize = 1;
const PART2_DELTA: usize = 999999;

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        self.map.push(line.chars().collect());
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let part1 = self.calculate_distances(PART1_DELTA);
        info!("[1] Sum of distances with delta {PART1_DELTA}: {part1}");

        let part2 = self.calculate_distances(PART2_DELTA);
        info!("[2] Sum of distances with delta {PART2_DELTA}: {part2}");

        Some((part1.to_string(), part2.to_string()))
    }
}
