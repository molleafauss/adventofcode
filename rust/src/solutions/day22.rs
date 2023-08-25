// What did I learn?
// quite straight-forward conversion.
// A lot of headaches flipping between i64 and usize (as many indexing only take usize).
// Used a slice for known-length data.

use std::str::FromStr;
use std::usize;
use crate::Solver;

pub(crate) struct Solution {
    is_map: bool,
    cube_size: usize,
    map: Vec<Vec<char>>,
    faces: [Face; 6],
    path: Vec<Move>,
}

// using i64 so that usize<->i64 should be "simpler" (although numbers are quire small)
const DIRS: [(i64, i64); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0)
];
const DIR_TEXT: [char; 4] = ['>', 'v', '<', '^'];

fn turn(dir: usize, val: i64) -> usize {
    let mut v = dir as i64 + val;
    let len = DIRS.len() as i64;
    if v >= len {
        v -= len;
    } else if v < 0 {
        v += len;
    }
    v as usize
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            is_map: true,
            cube_size: 0,
            map: Vec::new(),
            faces: [Face::empty(); 6],
            path: Vec::new(),
        }
    }

    fn add_to_map(&mut self, line: &str) {
        self.map.push(line.chars().collect());
    }

    fn find_faces(&mut self) {
        // there is (should be) always one face in row 0
        let mut next_row = 0;
        let mut next_col = 0;
        let mut face_id = 0;
        while face_id < self.faces.len() {
            if next_col < self.map[next_row].len() && self.map[next_row][next_col] != ' ' {
                self.faces[face_id].id = face_id + 1;
                self.faces[face_id].row = next_row;
                self.faces[face_id].col = next_col;
                self.faces[face_id].size = self.cube_size - 1;
                face_id += 1;
                println!("Found face {} at {}, {}", face_id, next_row, next_col)
            }
            next_col += self.cube_size;
            if self.map[next_row].len() < next_col {
                next_row += self.cube_size;
                next_col = 0;
            }
        }
        println!("Found {} faces", face_id);
    }

    fn add_facing(&mut self, line: &str) {
        let face_id = usize::from_str(&line[0..1]).unwrap() - 1;
        let mut parts = line[2..].split(",");
        let mut facing_id = 0;
        while facing_id < 4 {
            self.faces[face_id].facing[facing_id].0 = usize::from_str(parts.next().unwrap()).unwrap();
            self.faces[face_id].facing[facing_id].1 = match parts.next().unwrap() {
                "U" => 'U',
                "L" => 'L',
                "R" => 'R',
                "=" => '=',
                "None" => ' ',
                _ => panic!("Unrecognized facing in {}", line),
            };
            facing_id += 1;
        }
    }

    fn parse_path(&mut self, line: &str) {
        let mut count = 0;
        line.chars().for_each(|ch| {
            if ch.is_ascii_digit() {
                count = (count * 10) + ch.to_digit(10).unwrap();
            } else {
                self.path.push(Move::Walk(count));
                self.path.push(Move::Turn(ch));
                count = 0;
            }
        });
        self.path.push(Move::Walk(count));
    }

    fn part1_walk(&self, start: (usize, usize, usize), steps: u32) -> (usize, usize, usize) {
        // rows are ordered top->bottom: v moves down = rows + 1; ^ moves up = rows -1
        let mut pos = start;
        let mut walk = steps;
        while walk > 0 {
            let new_pos = if pos.2 == 0 || pos.2 == 2 {
                self.move_col(pos)
            } else {
                self.move_row(pos)
            };
            if self.map[new_pos.0][new_pos.1] == '#' {
                // found a wall, stop
                break;
            }
            walk -= 1;
            pos = new_pos;
        }
        assert_eq!(self.map[pos.0][pos.1], '.');
        // println!("Walk {} => [{}, {}, {}]", steps, pos.0, pos.1, pos.2);
        pos
    }

    fn part2_walk(&self, start: (usize, usize, usize), steps: u32) -> (usize, usize, usize) {
        let mut pos = start;
        let mut walk = steps;
        while walk > 0 {
            let new_pos = self.cube_walk(pos);
            if self.map[new_pos.0][new_pos.1] == '#' {
                // found a wall, stop
                break;
            }
            walk -= 1;
            pos = new_pos;
        }
        assert_eq!(self.map[pos.0][pos.1], '.');
        // println!("Walk {} => [{}, {}, {}]", steps, pos.0, pos.1, pos.2);
        pos
    }

    fn cube_walk(&self, pos: (usize, usize, usize)) -> (usize, usize, usize) {
        // only move one step
        let (r0, c0, dir) = pos;
        let dr = DIRS[dir].0;
        let dc = DIRS[dir].1;
        let face = self.in_face(r0, c0);
        let r = (r0 as i64 + dr) as usize;
        let c = (c0 as i64 + dc) as usize;
        if face.contains(r, c) {
            return (r, c, dir);
        }
        let adj = &face.facing[dir];
        if adj.1 == ' ' {
            println!("Crossing face {} / {}: ({}, {}, {}) => ({}, {}, {})",
                  face.id, adj.0, r0 + 1, c0 + 1, DIR_TEXT[dir], r + 1, c + 1, DIR_TEXT[dir]);
            return (r, c, dir);
        }
        let fadj = &self.faces[adj.0 - 1];
        // cross into new face based on old position
        let pos = fadj.cross(face.relative(r0, c0), dir, adj.1);
        assert!(fadj.contains(pos.0, pos.1));
        println!("Crossing face {} / {}: ({}, {}, {}) => ({}, {}, {})",
              face.id, adj.0, r0 + 1, c0 + 1, DIR_TEXT[dir], pos.0 + 1, pos.1 + 1, DIR_TEXT[pos.2]);
        pos
    }

    fn in_face(&self, r: usize, c: usize) -> &Face {
        let f = self.faces.iter().find(|f| f.contains(r, c));
        if f.is_none() {
            panic!("{r}, {c} not in any face?");
        }
        f.unwrap()
    }

    fn in_map(&self, row: usize, col: usize) -> bool {
        // ensure row is within boundaries (no need to check for <0 as usize can't be negative)
        if row >= self.map.len() {
            return false;
        }
        // ensure column is within boundaries of row (no need to check for <0 as usize can't be negative)
        if col >= self.map[row].len() {
            return false;
        }
        // if inside borders, returns true if not void space
        self.map[row][col] != ' '
    }

    fn move_col(&self, start: (usize, usize, usize)) -> (usize, usize, usize) {
        let (r, mut c, dir) = (start.0 as i64, start.1 as i64, start.2);
        let dc = DIRS[dir].1;
        c += dc;
        let row_len = self.map[r as usize].len() as i64;
        // keep moving column in same direction until there's either a floor (.) or a wall (#)
        while !self.in_map(r as usize, c as usize) {
            if c < 0 {
                c = row_len - 1;
            } else if c >= row_len {
                c = 0
            } else if self.map[r as usize][c as usize] == ' ' {
                c += dc;
            }
        }
        (r as usize, c as usize, dir)
    }

    fn move_row(&self, start: (usize, usize, usize)) -> (usize, usize, usize) {
        let (mut r, c, dir) = (start.0 as i64, start.1 as i64, start.2);
        let dr = DIRS[dir].0;
        r += dr;
        while !self.in_map(r as usize, c as usize) {
            if r < 0 {
                r = self.map.len() as i64 - 1;
            } else if r >= self.map.len() as i64 {
                r = 0;
            } else if c >= self.map[r as usize].len() as i64 {
                r += dr;
            } else if self.map[r as usize][c as usize] == ' ' {
                r += dr;
            }
        }
        (r as usize, c as usize, dir)
    }

    fn turn(&self, pos: (usize, usize, usize), turn: char) -> (usize, usize, usize) {
        let mut facing = pos.2 as i64;
        match turn {
            'R' => facing += 1,
            'L' => facing -= 1,
            _ => panic!("Invalid turn? {}", turn),
        }
        if facing < 0 {
            facing = 3;
        } else if facing > 3 {
            facing = 0;
        }
        // println!("Turn {turn} => [{}, {}, {}]", pos.0, pos.1, facing);
        (pos.0, pos.1, facing as usize)
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            self.is_map = false;
            return;
        }
        if self.is_map {
            self.add_to_map(line);
        } else if line.starts_with("CUBE SIZE ") {
            self.cube_size = usize::from_str(&line[10..]).unwrap();
            self.find_faces()
        } else if line.starts_with("CUBE FACE ") {
            self.add_facing(&line[10..]);
        } else {
            self.parse_path(line);
        }

    }

    fn solve(&mut self) {
        println!("Path: {} movements+turns", self.path.len());
        let mut pos = (0, self.map[0].iter().position(|ch| *ch == '.').unwrap(), 0);
        println!("Starting position: {:?}", pos);
        for act in &self.path {
            match act {
                Move::Walk(steps) => pos = self.part1_walk(pos, *steps),
                Move::Turn(dir) => pos = self.turn(pos, *dir),
            }
        }
        let password = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + pos.2;
        println!("[1] final position: {:?} => password {password}", pos);

        let mut pos = (0, self.map[0].iter().position(|ch| *ch == '.').unwrap(), 0);
        println!("==> Cube walk: starting position: {:?}", pos);
        for act in &self.path {
            match act {
                Move::Walk(steps) => pos = self.part2_walk(pos, *steps),
                Move::Turn(dir) => pos = self.turn(pos, *dir),
            }
        }
        let password = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + pos.2;
        println!("[2] final position: {:?} => password {password}", pos);
    }
}

#[derive(Copy, Clone)]
struct Face {
    id: usize,
    row: usize,
    col: usize,
    size: usize,
    facing: [(usize, char); 4],
}

impl Face {
    fn empty() -> Face {
        Face {
            id: 0,
            row: 0,
            col: 0,
            size: 0,
            facing: [(0, ' '); 4],
        }
    }

    fn contains(&self, row: usize, col: usize) -> bool {
        self.row <= row && row <= self.row + self.size && self.col <= col && col <= self.col + self.size
    }

    fn relative(&self, row: usize, col: usize) -> (usize, usize) {
        (row - self.row, col - self.col)
    }

    fn cross(&self, pos: (usize, usize), dir: usize, rotate: char) -> (usize, usize, usize) {
        // pos[0] is diff in rows, pos[1] is diff in columns - one should be not relevant
        match (dir, rotate) {
            // crossing from the right to the top border
            (0, 'R') => (self.row, self.col + self.size - pos.0, turn(dir, 1)),
            // crossing from the right to the bottom border
            (0, 'L') => (self.row + self.size, self.col + pos.0, turn(dir, -1)),
            // crossing from the right to the right border (flipping)
            (0, 'U') => (self.row + self.size - pos.0, self.col + self.size, turn(dir, 2)),
            // crossing from the bottom to the right border
            (1, 'R') => (self.row + pos.1, self.col + self.size, turn(dir, 1)),
            // crossing from the bottom to the left border
            (1, 'L') => (self.row + pos.1, self.col, turn(dir, -1)),
            // crossing from the bottom to the bottom border
            (1, 'U') => (self.row + self.size, self.col + self.size - pos.1, turn(dir, 2)),
            // bottom to top - no change in dir
            (1, '=') => (self.row, self.col + pos.1, dir),
            // crossing from the left to the bottom border
            (2, 'R') => (self.row + self.size, self.col + pos.0, turn(dir, 1)),
            // crossing from the left to the top border
            (2, 'L') => (self.row, self.col + pos.0, turn(dir, -1)),
            // crossing from the left to the left border
            (2, 'U') => (self.row + self.size - pos.0, self.col, turn(dir, 2)),
            // crossing from the top to the right border
            (3, 'R') => (self.row + pos.1, self.col, turn(dir, 1)),
            // crossing from the top to the left border
            (3, 'L') => (self.row + pos.1, self.col + self.size, turn(dir, -1)),
            // crossing from the top to the top border
            (3, 'U') => (self.row, self.col + self.size - pos.1, turn(dir, 2)),
            // top to bottom - no change in dir
            (3, '=') => (self.row + self.size, self.col + pos.1, dir),
            _ => panic!("Cannot determine where to go for {dir}, {rotate} / {:?}", pos),
        }
    }
}

enum Move {
    Walk(u32),
    Turn(char),
}