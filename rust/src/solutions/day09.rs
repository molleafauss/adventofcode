// What did I learn?
// incapsulate some of the actions into structs, implementing Display, using a HashSet with int structs

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::Solver;

pub(crate) struct Solution {
    movements: Vec<Movement>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            movements: Vec::new(),
        }
    }

    fn move_rope(&mut self, rope_length: usize) -> usize {
        println!("Moving rope with length {rope_length}");
        let mut rope = Rope::with_length(rope_length);
        let mut visited = HashSet::new();
        self.movements.iter().for_each(|movement| {
            // println!("Moving {} {}", movement.dir, movement.moves);
            (0..movement.moves).for_each(|_| {
                rope.move_head(movement.dir);
                (0..rope_length - 1).for_each(|pos| rope.adjust_rope(pos));
                visited.insert(rope.tail().clone());
            });
        });
        visited.len()
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let (dir, count) = line.split_once(" ").unwrap();
        self.movements.push(Movement {
            dir: char::from_str(dir).unwrap(),
            moves: u32::from_str(count).unwrap(),
        })
    }

    fn solve(&mut self) {
        let part1 = self.move_rope(2);
        println!("[1] Tail visited {part1} places");
        let part2 = self.move_rope(10);
        println!("[2] Tail visited {part2} places");
    }
}

struct Movement {
    dir: char,
    moves: u32,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct GridPos {
    x: i32,
    y: i32,
}

impl GridPos {
    fn of(x: i32, y: i32) -> GridPos {
        GridPos {x, y}
    }

    fn distance(&self, other: &GridPos) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    fn move_by(&mut self, other: &GridPos) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Display for GridPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

const MOVE_U : GridPos = GridPos { x: 0, y: 1 };
const MOVE_D : GridPos = GridPos { x: 0, y: -1 };
const MOVE_R : GridPos = GridPos { x: 1, y: 0 };
const MOVE_L : GridPos = GridPos { x: -1, y: 0 };

struct Rope {
    knots: Vec<GridPos>
}

impl Rope {
    fn with_length(len: usize) -> Rope {
        Rope { knots: vec![GridPos{x: 0, y: 0}; len] }
    }

    fn tail(&self) -> &GridPos {
        self.knots.last().unwrap()
    }

    fn move_head(&mut self, dir: char) {
        match dir {
            'U' => self.knots.first_mut().unwrap().move_by(&MOVE_U),
            'D' => self.knots.first_mut().unwrap().move_by(&MOVE_D),
            'R' => self.knots.first_mut().unwrap().move_by(&MOVE_R),
            'L' => self.knots.first_mut().unwrap().move_by(&MOVE_L),
            _ => panic!("Invalid movement direction {}", dir),
        }
    }

    fn adjust_rope(&mut self, pos: usize) {
        let head = self.knots.get(pos).unwrap();
        // can't get tail as mutable or head.distance(tail) will fail due to two different borrows
        let tail = self.knots.get(pos + 1).unwrap();
        let (delta_x, delta_y) = head.distance(tail);
        if delta_x.abs() <= 1 && delta_y.abs() <= 1 {
            // within range - nothing to do
            return;
        }

        let mut tail_move = GridPos::of(0, 0);
        if delta_x == 0 {
            move_y = if delta_y > 0  { 1 } else { -1 };
        } else if delta_y == 0 {
            move_x = if delta_x > 0  { 1 } else { -1 };
        } else if delta_x.abs() == 2 && delta_y.abs() == 2 {
            move_x = if delta_x > 0  { 1 } else { -1 };
            move_y = if delta_y > 0  { 1 } else { -1 };
        } else if delta_x.abs() > delta_y.abs() {
            move_x = if delta_x > 0  { 1 } else { -1 };
            move_y = delta_y;
        } else if delta_y.abs() > delta_x.abs() {
            move_x = delta_x;
            move_y = if delta_y > 0  { 1 } else { -1 };
        } else {
            panic!("Invalid head-tail distance: {} => {} : ({delta_x}, {delta_y})", head, tail);
        }
        // now get again tail and mutate it
        self.knots.get_mut(pos + 1).unwrap().move_by(&tail_move);
    }
}