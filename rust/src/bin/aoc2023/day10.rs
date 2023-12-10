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

    fn next_step(&self, path: &mut Path) {
        let tunnel = self.map.get(&path.pos).unwrap();
        let next = match path.pos.add(&tunnel.dirs[0]) == path.prev {
            true => &tunnel.dirs[1],
            false => &tunnel.dirs[0],
        };
        let next_pos = path.pos.add(next);
        if !self.map.contains_key(&next_pos) {
            debug!("Should ignore this - dead end?");
        }
        path.prev = path.pos.clone();
        path.pos = next_pos;
        path.distance += 1;
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
                    paths.push(Path { prev: self.start.clone(), pos, distance: 1});
                }
            }
        }
        debug!("Starting with the following paths [{}]: {:?}", paths.len(), paths);

        let mut max_distance = 0;
        while !paths.is_empty() {
            paths.iter_mut().for_each(|path| self.next_step(path));
            // find pairs
            let mut i = 0;
            // need at least 2
            while i + 1 < paths.len() {
                let one = paths.get(i).unwrap();
                let second = paths.iter().enumerate()
                    .position(|(q, p)| q > i && p.pos == one.pos && p.distance == one.distance);
                if let Some(p) = second {
                    info!("Found matching paths ({i}, {}) => [max {max_distance}] {:?}", p + i + 1, one);
                    if max_distance < one.distance {
                        max_distance = one.distance;
                    }
                    paths.remove(p);
                    paths.remove(i);
                    continue;
                }
                i += 1;
            }
        }
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

#[derive(Debug)]
struct Path {
    prev: GridPos,
    pos: GridPos,
    distance: u32,
}