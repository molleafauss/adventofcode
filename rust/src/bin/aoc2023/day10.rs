// https://adventofcode.com/2023/day/10

use std::collections::HashMap;

use log::{debug, info};

use adventofcode::grid::{ALL_ORTHOGONAL, GridPos, MOVE_D, MOVE_L, MOVE_R, MOVE_U};
use adventofcode::Solver;

pub struct Solution {
    map: HashMap<GridPos, &'static Tunnel>,
    start: GridPos,
    width: usize,
    height: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            map: HashMap::new(),
            start: GridPos::of(0, 0),
            width: 0,
            height: 0,
        }
    }

    fn next_step(&self, path: &mut Vec<GridPos>) {
        let cur_pos = path.last().unwrap();
        let prev_pos = path.get(path.len() - 2).unwrap();

        let tunnel = self.map.get(&cur_pos).unwrap();
        let next = match cur_pos.add(&tunnel.dirs[0]) == *prev_pos {
            true => &tunnel.dirs[1],
            false => &tunnel.dirs[0],
        };
        let next_pos = cur_pos.add(next);
        if !self.map.contains_key(&next_pos) {
            debug!("Should ignore this - dead end?");
        }
        path.prev = path.pos.clone();
        path.pos = next_pos;
        path.distance += 1;
        path.push(next_pos);
    }

    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if self.width == 0 {
            self.width = line.len()
        } else {
            assert_eq!(self.width, line.len());
        }

        line.chars().enumerate().for_each(|(idx, ch)| {
            if ch == '.' {
                return;
            }

            if ch == 'S' {
                self.start.row = self.height as i64;
                self.start.col = idx as i64;
                return;
            }

            let tunnel = TUNNELS.iter().find(|tunnel| tunnel.glyph == ch);
            if tunnel.is_none() {
                panic!("Invalid charachter at ({}, {}) => {}", idx, self.height, ch);
            }
            self.map.insert(GridPos::of(idx as i64, self.height as i64), tunnel.unwrap());
        });


        self.height += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("Map is ({},{}), start at {}", self.width, self.height, self.start);
        // try to follow all paths from the start (max 4)
        let mut paths = Vec::new();
        // find all surrounding paths from start
        for dir in &ALL_ORTHOGONAL {
            let pos = self.start.add(dir);
            let map_pos = self.map.get(&pos);
            if let Some(tunnel) = map_pos {
                // check if tunnel has a "matching" facing with the dir I am trying to move
                if facing(&tunnel.dirs[0], dir) || facing(&tunnel.dirs[1], dir) {
                    paths.push(vec![self.start.clone(), pos]);
                }
            }
        }
        debug!("Starting with the following paths [{}]: {:?}", paths.len(), paths);
        // I should only have two paths
        assert_eq!(paths.len(), 2);

        let mut one = paths.remove(0);
        let mut two = paths.remove(0);

        while one.last().unwrap() != two.last().unwrap() {
            self.next_step(&mut one);
            self.next_step(&mut two);
        }
        let max_distance = one.len() - 1;
        info!("[1] Found max distance: {}", max_distance);

        Some((max_distance.to_string(), "".to_string()))
    }
}

fn facing(from: &GridPos, to: &GridPos) -> bool {
    return (*from == MOVE_L && *to == MOVE_R)
        || (*from == MOVE_R && *to == MOVE_L)
        || (*from == MOVE_U && *to == MOVE_D)
        || (*from == MOVE_D && *to == MOVE_U);
}

struct Tunnel {
    glyph: char,
    dirs: [GridPos; 2],
}

static TUNNELS: [Tunnel; 6] = [
    Tunnel { glyph: '|', dirs: [MOVE_U, MOVE_D] },
    Tunnel { glyph: '-', dirs: [MOVE_L, MOVE_R] },
    Tunnel { glyph: 'L', dirs: [MOVE_D, MOVE_R] },
    Tunnel { glyph: 'J', dirs: [MOVE_D, MOVE_L] },
    Tunnel { glyph: '7', dirs: [MOVE_U, MOVE_L] },
    Tunnel { glyph: 'F', dirs: [MOVE_U, MOVE_R] },
];
