// What did I learn?
// first time using a reference in a struct- it's taken from a 'static struct so it works.
// nothing major, I've changed the pieces to be references instead of copies.
// had to refactor GridPos to i64 as numbers are getting bigger here.

use std::collections::HashMap;
use log::{debug, info};
use once_cell::sync::Lazy;
use adventofcode::grid::GridPos;
use adventofcode::Solver;

const MAX_ROCKS_P1: u64 = 2022;
const MAX_ROCKS_P2: u64 = 1_000_000_000_000;
const WIDTH: i64 = 7;
const EMPTY: u8 = '.' as u8;
const BLOCK: u8 = '#' as u8;
const LEFT: u8 = '<' as u8;
const RIGHT: u8 = '>' as u8;

pub(crate) struct Solution {
    winds: Vec<u8>,
    wind_pos: usize,
    height1: Option<i64>,
    height2: Option<i64>,
    rocks: u64,
    max_height: i64,
    chamber: Vec<Vec<u8>>,
    status: HashMap<CacheKey, (u64, i64)>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            winds: Vec::new(),
            wind_pos: 0,
            height1: None,
            height2: None,
            rocks: 0,
            max_height: 0,
            chamber: Vec::new(),
            status: HashMap::new(),
        }
    }
}

impl Solution {
    fn drop_piece(&mut self, idx: usize) {
        let mut piece = Piece { shape: &SHAPES[idx], pos: GridPos::of(2, self.max_height + 3) };
        let mut resting = false;
        while !resting {
            let wind = self.winds[self.wind_pos];
            self.wind_pos += 1;
            if self.wind_pos >= self.winds.len() {
                self.wind_pos = 0;
            }
            // move left/right if possible
            let mut dir = 0;
            if wind == LEFT && self.clear_x(&piece, -1) {
                dir = -1;
            } else if wind == RIGHT && self.clear_x(&piece, 1) {
                dir = 1;
            }
            piece.pos.col += dir;
            // move down - flag resting if it can't move down
            let down = if self.clear_y(&piece) { -1 } else { 0 };
            piece.pos.row += down;
            resting = down == 0;
        }
        self.place(piece);
    }

    fn clear_x(&self, piece: &Piece, dir: i64) -> bool {
        if piece.pos.col + dir < 0 || piece.pos.col + piece.shape.width + dir > WIDTH {
            return false;
        }
        // check if the shape is over the resting rocks
        if piece.pos.row > self.max_height {
            return true;
        }
        // check if inside the chamber the shape can move in the expected dir
        let mut h = 0;
        while h < piece.shape.height && piece.pos.row + h < self.max_height {
            let row = &self.chamber[(piece.pos.row + h) as usize];
            let blk = &piece.shape.lines[(piece.shape.height - 1 - h) as usize];
            for x in 0..piece.shape.width {
                if blk[x as usize] == BLOCK && row[(piece.pos.col + x + dir) as usize] == BLOCK {
                    return false;
                }
            }
            h += 1;
        }
        true
    }

    fn clear_y(&self, piece: &Piece) -> bool {
        if piece.pos.row - 1 > self.max_height {
            return true;
        }
        if piece.pos.row == 0 {
            return false;
        }
        // bottom row of the piece is near the bottom row of the chamber - see if there's any '#' touching
        let mut h = 0;
        while h < piece.shape.height && piece.pos.row + h <= self.max_height {
            let rocks_below = &self.chamber[(piece.pos.row + h - 1) as usize];
            let blk = &piece.shape.lines[(piece.shape.height - 1 - h) as usize];
            for w in 0..piece.shape.width {
                if rocks_below[(piece.pos.col + w) as usize] == BLOCK && blk[w as usize] == BLOCK {
                    return false;
                }
            }
            h += 1;
        }
        true
    }

    fn place(&mut self, piece: Piece) {
        // place bottom-up
        let mut h = 0;
        while h < piece.shape.height {
            // reverse lookup
            let blk = &piece.shape.lines[(piece.shape.height - 1 - h) as usize];
            let r = piece.pos.row + h;
            h += 1;
            if r >= self.max_height {
                self.max_height += 1;
                self.chamber.push(vec![EMPTY; WIDTH as usize]);
            }
            let row = &mut self.chamber[r as usize];
            for c in 0..piece.shape.width {
                if blk[c as usize] == BLOCK {
                    row[(piece.pos.col + c) as usize] = BLOCK;
                }
            }
        }
    }

    fn detect_cycle(&mut self, piece: usize) {
        let mut column_status = vec![0; WIDTH as usize];
        let mut y = self.max_height;
        while column_status.contains(&0) && y > 0 {
            y -= 1;
            let row = &self.chamber[y as usize];
            for i in 0..WIDTH as usize {
                if row[i] == BLOCK && column_status[i] == 0 {
                    column_status[i] = self.max_height - y;
                }
            }
        }
        let cache_key = CacheKey(self.wind_pos, piece, column_status.clone());
        if !self.status.contains_key(&cache_key) {
            self.status.insert(cache_key, (self.rocks, self.max_height));
            return;
        }

        let (old_rocks, old_height) = &self.status[&cache_key];
        debug!("Found cycle: {} => {}: {:?}", old_rocks, self.rocks, cache_key);
        if self.height1.is_none() {
            self.height1 = Some(self.calculate_height(MAX_ROCKS_P1, *old_rocks, *old_height));
        }
        if self.height2.is_none() {
            self.height2 = Some(self.calculate_height(MAX_ROCKS_P2, *old_rocks, *old_height));
        }
    }

    fn calculate_height(&self, target: u64, old_rocks: u64, old_height: i64) -> i64 {
        let cycle = self.rocks - old_rocks;
        let height_diff = self.max_height - old_height;
        let num_cycles = (target - self.rocks) / cycle;
        let mut rocks = self.rocks + num_cycles * cycle;
        let mut max_height = self.max_height + num_cycles as i64 * height_diff;
        if rocks < target {
            let delta = target - rocks;
            // find the entry for the entry we loop back + the missing rocks to reach the goal - add the extra
            // height to the result
            let (_, delta_height) = self.status.values()
                .filter(|(r, _h)| *r == old_rocks + delta)
                .next().unwrap();
            rocks += delta;
            assert_eq!(rocks, target);
            max_height += delta_height - old_height;
        }
        max_height
    }
}


impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        self.winds = line.as_bytes().into();
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let mut piece = 0;
        while self.height1.is_none() || self.height2.is_none() {
            self.rocks += 1;
            self.drop_piece(piece);
            piece += 1;
            if piece >= SHAPES.len() {
                piece = 0;
            }
            self.detect_cycle(piece);
            if self.rocks == MAX_ROCKS_P1 {
                self.height1 = Some(self.max_height);
            } else if self.rocks == MAX_ROCKS_P2 {
                self.height2 = Some(self.max_height);
            }
            if self.rocks % 1000 == 0 {
                debug!("{} rocks dropped", self.rocks);
            }
        }
        info!("[1] Chamber height: {}", self.height1.unwrap());
        info!("[2] Chamber height: {}", self.height2.unwrap());
        Some((self.height1.unwrap().to_string(), self.height2.unwrap().to_string()))
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct CacheKey(usize, usize, Vec<i64>);

struct Piece<'a> {
    shape: &'a Shape,
    pos: GridPos,
}

static SHAPES: Lazy<Vec<Shape>> = Lazy::new(|| {
    vec![
        Shape::create(0, Vec::from(["####"])),
        Shape::create(1, Vec::from([".#.", "###", ".#."])),
        Shape::create(2, Vec::from(["..#", "..#", "###"])),
        Shape::create(3, Vec::from(["#", "#", "#", "#"])),
        Shape::create(4, Vec::from(["##", "##"]))
    ]
});

struct Shape {
    _id: u8,
    lines: Vec<Vec<u8>>,
    width: i64,
    height: i64,
}

impl Shape {
    fn create(_id: u8, parts: Vec<&str>) -> Shape {
        let height = parts.len() as i64;
        let width = parts[0].len() as i64;
        let lines = parts.iter()
            .map(|l| Vec::from(l.as_bytes()))
            .collect();
        Shape {
            _id,
            lines,
            width,
            height,
        }
    }
}