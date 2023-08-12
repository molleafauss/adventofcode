use std::cmp::max;
use std::collections::HashMap;
use std::i32;
use std::str::FromStr;
use crate::grid::GridPos;
use crate::Solver;

pub(crate) struct Solution {
    scan: HashMap<GridPos, char>,
    max_y: i32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            scan: HashMap::new(),
            max_y: 0
        }
    }
}

const START: (i32, i32) = (500, 0);

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        let parts: Vec<(i32, i32)> = line.split(" -> ")
            .map(|part| {
                let (a, b) = part.split_once(",").unwrap();
                (i32::from_str(a).unwrap(), i32::from_str(b).unwrap())
            }).collect();
        for i in 0..parts.len() - 1 {
            let (x0, y0) = parts[i];
            let (x1, y1) = parts[i + 1];
            println!("Tracing line: ({x0}, {y0}) <=> ({x1}, {y1})");
            if x0 == x1 {
                // horizontal line
                match y1 > y0 {
                    true => y0..y1 + 1,
                    false => y1..y0 + 1,
                }.for_each(|y| { self.scan.insert(GridPos::of(x0, y), '#'); });
            } else if y0 == y1 {
                // vertical line
                match x1 > x0 {
                    true => x0..x1 + 1,
                    false => x1..x0 + 1,
                }.for_each(|x| { self.scan.insert(GridPos::of(x, y0), '#'); });
            } else {
                panic!("Not a straight line? ({x0}, {y0}) <=> ({x1}, {y1})")
            }
            self.max_y = max(self.max_y, max(y0, y1));
        }
    }

    fn solve(&mut self) {
        println!("Max y {} / walls {}", self.max_y, self.scan.len());
        // part 1
        let mut keep_dripping = true;
        let mut sand = 0;
        while keep_dripping {
            let (mut x, mut y) = START;
            // println!("Starting at {x}, {y}");
            while y <= self.max_y {
                if !self.scan.contains_key(&GridPos::of(x, y + 1)) {
                    // println!("fall down => ({x}, {})", y + 1);
                    y += 1;
                } else if !self.scan.contains_key(&GridPos::of(x - 1, y + 1)) {
                    // println!("fall left => ({}, {})", x - 1, y + 1);
                    y += 1;
                    x -= 1;
                } else if !self.scan.contains_key(&GridPos::of(x + 1, y + 1)) {
                    // println!("fall right => ({}, {})", x + 1, y + 1);
                    y += 1;
                    x += 1;
                } else {
                    // println!("Sand found resting place in ({x}, {y}): {sand}");
                    sand += 1;
                    self.scan.insert(GridPos::of(x, y),'o');
                    break;
                }
            }
            // println!("Stopped at {y} - {}", self.scan.len());
            // if reached the abyss - stop
            keep_dripping = y <= self.max_y;
        }
        println!("[1] Sand resting: {sand}");

        self.scan.retain(|_, v| *v == '#');
        println!("Part 2 - starting with {}", self.scan.len());

        // part 2
        self.max_y += 2;
        keep_dripping = true;
        sand = 0;
        let start_pos = GridPos::of(START.0, START.1);
        while keep_dripping {
            let (mut x, mut y) = START;
            // println!("Starting at {x}, {y}");
            while y <= self.max_y - 1 {
                if !self.scan.contains_key(&GridPos::of(x, y + 1)) && y + 1 < self.max_y {
                    // println!("fall down => ({x}, {})", y + 1);
                    y += 1;
                } else if !self.scan.contains_key(&GridPos::of(x - 1, y + 1)) && y + 1 < self.max_y {
                    // println!("fall left => ({}, {})", x - 1, y + 1);
                    y += 1;
                    x -= 1;
                } else if !self.scan.contains_key(&GridPos::of(x + 1, y + 1)) && y + 1 < self.max_y {
                    // println!("fall right => ({}, {})", x + 1, y + 1);
                    y += 1;
                    x += 1;
                } else if y + 1 == self.max_y {
                    // println!("Sand found resting place in ({x}, {y}): {sand}");
                    sand += 1;
                    self.scan.insert(GridPos::of(x, y),'o');
                    break;
                } else {
                    // println!("Sand found resting place in ({x}, {y}): {sand}");
                    sand += 1;
                    self.scan.insert(GridPos::of(x, y),'o');
                    break;
                }
            }
            // println!("Stopped at {y} - {}", self.scan.len());
            // if reached the abyss - stop
            keep_dripping = !self.scan.contains_key(&start_pos);
        }
        println!("[2] Sand resting: {sand}");
    }
}
