// What did I learn?

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
            println!("Face {}: {}/'{}'", face_id + 1, self.faces[face_id].facing[facing_id].0, self.faces[face_id].facing[facing_id].1);
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

    fn plane_walk(&self, start: (usize, usize, usize), steps: u32) -> (usize, usize, usize) {
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

    fn in_map(&self, row: usize, col: usize) -> bool {
        // ensure row is within boundaries
        if row < 0 || row >= self.map.len() {
            return false;
        }
        // ensure column is within boundaries of row
        if col < 0 || col >= self.map[row].len() {
            return false;
        }
        // if inside borders, returns true if not void space
        self.map[row][col] != ' '
    }

    fn move_col(&self, start: (usize, usize, usize)) -> (usize, usize, usize) {
        let (mut r, mut c, mut dir) = (start.0 as i64, start.1 as i64, start.2);
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
        let (mut r, mut c, mut dir) = (start.0 as i64, start.1 as i64, start.2);
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

    fn face_walk(&self, pos: (usize, usize, usize), steps: u32) -> (usize, usize, usize) {
        pos
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
                Move::Walk(steps) => pos = self.plane_walk(pos, *steps),
                Move::Turn(dir) => pos = self.turn(pos, *dir),
            }
        }
        let password = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + pos.2;
        println!("[1] final position: {:?} => password {password}", pos);

        let mut pos = (0, self.map[0].iter().position(|ch| *ch == '.').unwrap(), 0);
        println!("==> Cube walk: starting position: {:?}", pos);
        for act in &self.path {
            match act {
                Move::Walk(steps) => pos = self.face_walk(pos, *steps),
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
}

enum Move {
    Walk(u32),
    Turn(char),
}