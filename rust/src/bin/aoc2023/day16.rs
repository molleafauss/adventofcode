// https://adventofcode.com/2023/day/16

use std::collections::{HashMap, HashSet};
use log::{debug, info};
use adventofcode::grid::{GridPos, MOVE_D, MOVE_L, MOVE_R, MOVE_U};
use adventofcode::Solver;

pub struct Solution {
    mirrors: HashMap<GridPos, char>,
    width: usize,
    height: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            mirrors: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    fn print_lits(&self, tiles: &HashSet<GridPos>) {
        for row in 0..self.height {
            let text: String = (0..self.height)
                .map(|col| if tiles.contains(&GridPos::of(col as i64, row as i64)) { '#' } else { '.' })
                .collect();
            println!("{text}");
        }
    }

    fn next_pos(&self, pos: GridPos, dir: GridPos) -> Option<(GridPos, GridPos)> {
        let next_pos = pos.add(&dir);
        if next_pos.row < 0 || next_pos.row >= self.width as i64
            || next_pos.col < 0 || next_pos.col >= self.height as i64 {
            // beam is out of map - stop following it
            return None;
        } else {
            Some((next_pos, dir.clone()))
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if self.width == 0 {
            self.width = line.len();
        } else {
            assert_eq!(line.len(), self.width);
        }
        line.chars().enumerate()
            .filter(|(_, ch)| *ch != '.')
            .for_each(|(pos, ch)| {
                self.mirrors.insert(GridPos::of(pos as i64, self.height as i64), ch);
            });
        self.height += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        debug!("Found {} mirrors, Map size {}x{}", self.mirrors.len(), self.width, self.height);

        // beams - start with the initial spot (top left moving right) - continue until empty
        let mut beams = vec![(GridPos::of(-1, 0), MOVE_R)];
        // tiles - add every new spot a beam passes through, duplicates handled by the set
        let mut tiles: HashSet<GridPos> = [GridPos::of(0, 0)].into_iter().collect();
        // set of starting point - whenever a beam split add it here - don't add it again if we
        // passed through the same
        let mut starts: HashSet<(GridPos, GridPos)> = vec![(GridPos::of(0, 0), MOVE_R)].into_iter().collect();

        while !beams.is_empty() {
            let (pos, dir) = beams.remove(0);
            let next = self.next_pos(pos, dir);
            if next.is_none() {
                // either out of map or back to a place it was already - drop this
                continue;
            }
            let (next_pos, dir) = next.unwrap();
            let mirror = self.mirrors.get(&next_pos);
            tiles.insert(next_pos.clone());
            if mirror.is_none() {
                beams.push((next_pos, dir));
                continue;
            }
            // mega match determining next direction
            match (&dir, mirror.unwrap()) {
                // energy from the right
                (&MOVE_R, '-') => beams.push((next_pos, MOVE_R)),
                (&MOVE_R, '/') => beams.push((next_pos, MOVE_D)),
                (&MOVE_R, '\\') => beams.push((next_pos, MOVE_U)),
                (&MOVE_R, '|') => {
                    let fork1 = (next_pos.clone(), MOVE_U);
                    if !starts.contains(&fork1) {
                        beams.push(fork1.clone());
                        starts.insert(fork1.clone());
                    }
                    let fork2 = (next_pos, MOVE_D);
                    if !starts.contains(&fork2) {
                        beams.push(fork2.clone());
                        starts.insert(fork2);
                    }
                },
                // energy from the left
                (&MOVE_L, '-') => beams.push((next_pos, MOVE_L)),
                (&MOVE_L, '/') => beams.push((next_pos, MOVE_U)),
                (&MOVE_L, '\\') => beams.push((next_pos, MOVE_D)),
                (&MOVE_L, '|') => {
                    let fork1 = (next_pos.clone(), MOVE_U);
                    if !starts.contains(&fork1) {
                        beams.push(fork1.clone());
                        starts.insert(fork1.clone());
                    }
                    let fork2 = (next_pos, MOVE_D);
                    if !starts.contains(&fork2) {
                        beams.push(fork2.clone());
                        starts.insert(fork2);
                    }
                },
                // energy from the top
                (&MOVE_U, '-') => {
                    let fork1 = (next_pos.clone(), MOVE_R);
                    if !starts.contains(&fork1) {
                        beams.push(fork1.clone());
                        starts.insert(fork1.clone());
                    }
                    let fork2 = (next_pos, MOVE_L);
                    if !starts.contains(&fork2) {
                        beams.push(fork2.clone());
                        starts.insert(fork2);
                    }
                },
                (&MOVE_U, '/') => beams.push((next_pos, MOVE_L)),
                (&MOVE_U, '\\') => beams.push((next_pos, MOVE_R)),
                (&MOVE_U, '|') => beams.push((next_pos.clone(), MOVE_U)),
                // energy from the bottom
                (&MOVE_D, '-') => {
                    let fork1 = (next_pos.clone(), MOVE_R);
                    if !starts.contains(&fork1) {
                        beams.push(fork1.clone());
                        starts.insert(fork1.clone());
                    }
                    let fork2 = (next_pos, MOVE_L);
                    if !starts.contains(&fork2) {
                        beams.push(fork2.clone());
                        starts.insert(fork2);
                    }
                },
                (&MOVE_D, '/') => beams.push((next_pos, MOVE_R)),
                (&MOVE_D, '\\') => beams.push((next_pos, MOVE_L)),
                (&MOVE_D, '|') => beams.push((next_pos.clone(), MOVE_D)),
                (_, ch) => panic!("Invalid combination direction {:?} - mirror {ch}", dir),
            }
        }
        info!("[1] Found {} energized tiles", tiles.len());

        Some((tiles.len().to_string(), "".to_string()))
    }
}

