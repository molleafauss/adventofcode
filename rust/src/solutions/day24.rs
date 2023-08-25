// What did I learn?
// mostly smooth sailing, some clones needed to make borrow checker happy

use std::collections::{HashSet, VecDeque};
use crate::grid::{GridPos, MOVE_D, MOVE_L, MOVE_R, MOVE_U};
use crate::Solver;

pub(crate) struct Solution {
    height: i64,
    width: i64,
    entry: GridPos,
    exit: GridPos,
    blizzards: Vec<Blizzard>,
    bliz_time: Vec<HashSet<GridPos>>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            height: 0,
            width: 0,
            entry: GridPos::of(0, 0),
            exit: GridPos::of(0, 0),
            blizzards: Vec::new(),
            bliz_time: Vec::new(),
        }
    }

    fn blizzards_at_time(&mut self, t: i64) -> &HashSet<GridPos> {
        if t < self.bliz_time.len() as i64 {
            return &self.bliz_time[t as usize]
        }
        let bliz_pos: HashSet<GridPos> = self.blizzards.iter().map(|b| GridPos {
                row: move_wrap(b.pos.row, b.dir.row * t, (1, self.height - 2)),
                col: move_wrap(b.pos.col, b.dir.col * t, (1, self.width - 2)),
            }).collect();
        self.bliz_time.push(bliz_pos);
        &self.bliz_time[t as usize]
    }

    fn find_path(&mut self, entry: GridPos, exit: GridPos, t: i64) -> i64 {
        println!("Finding path {entry} => {exit} starting at {t}");
        let mut steps = VecDeque::from([(entry, t)]);
        let mut visited = HashSet::new();
        // need copies to avoid second borrow:
        let map_entry = self.entry.clone();
        let map_exit = self.exit.clone();
        let map_width = self.width;
        let map_height = self.height;
        loop {
            let (pos, t) = steps.pop_front().unwrap();
            if pos == exit {
                // one less - minutes starting at "0"
                return t - 1;
            }
            if !visited.insert((pos.clone(), t)) {
                continue;
            }
            let blizzards = self.blizzards_at_time(t);
            for dir in &[MOVE_D, MOVE_R, MOVE_U, MOVE_L] {
                let new_pos = pos.add(dir);
                if blizzards.contains(&new_pos) {
                    continue;
                }
                // check if it's entry or exit
                let mut valid = new_pos == map_entry || new_pos == map_exit;
                // check for bounds
                valid |= (0 < new_pos.row && new_pos.row < map_height - 1) && (0 < new_pos.col && new_pos.col < map_width - 1);
                if !valid {
                    continue;
                }
                // println!("({}, {}), {t} -> ({}, {})", pos.row, pos.col, new_pos.row, new_pos.col);
                steps.push_back((new_pos, t + 1));
            }
            // check if next round pos it's empty
            if !blizzards.contains(&pos) {
                // println!("({}, {}), {t} -> ({}, {}) [wait]", pos.row, pos.col, pos.row, pos.col);
                steps.push_back((pos, t + 1));
            }
        }
    }
}

fn move_wrap(val: i64, delta: i64, bounds: (i64, i64)) -> i64 {
    let mut out = val + delta;
    let w = bounds.1 - bounds.0 + 1;
    while out < bounds.0 {
        out += w;
    }
    while out > bounds.1 {
        out -= w;
    }
    out
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if self.height == 0 {
            self.width = line.len() as i64;
            self.entry = GridPos::of(line.find('.').unwrap() as i64, 0);
            assert!(self.entry.col > 0 && self.entry.col < self.width);
        }
        line.chars().enumerate().for_each(|(pos, ch)| {
            if ch != '.' && ch != '#' {
                self.blizzards.push(Blizzard::new(GridPos::of(pos as i64, self.height), ch));
            }
        });
        if let Some(val) = line.find('.') {
            self.exit = GridPos::of(val as i64, self.height);
        }
        self.height += 1;
    }

    fn solve(&mut self) {
        println!("Tracing path from {:?} => {:?}", self.entry, self.exit);
        self.blizzards_at_time(0);
        let mut t = self.find_path(self.entry.clone(), self.exit.clone(), 0);
        println!("[1] Found exit in: {t}");
        t = self.find_path(self.exit.clone(), self.entry.clone(), t);
        t = self.find_path(self.entry.clone(), self.exit.clone(), t);
        println!("[2] Total time: {t}");
    }
}

struct Blizzard {
    pos: GridPos,
    dir: GridPos,
}

impl Blizzard {
    fn new(pos: GridPos, ch: char) -> Blizzard {
        let dir = match ch {
            '>' => MOVE_R,
            '<' => MOVE_L,
            '^' => MOVE_D,
            'v' => MOVE_U,
            _ => panic!("Invalid direction: {ch}"),
        };
        Blizzard {
            pos,
            dir,
        }
    }
}