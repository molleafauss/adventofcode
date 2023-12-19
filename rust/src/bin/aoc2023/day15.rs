// https://adventofcode.com/2023/day/15

use log::info;

use adventofcode::Solver;

pub struct Solution {
    part1: usize,
    boxes: Vec<Vec<Lens>>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        let boxes = (0..256).map(|_| Vec::new()).collect();
        Solution {
            part1: 0,
            boxes
        }
    }

    fn execute(&mut self, op: &str) -> usize {
        let mut check = 0;
        let mut label = String::new();
        let mut box_id = 0;
        let mut box_pos = None;
        for ch in op.chars() {
            if ch == '-' {
                // remove existing operation with the given label
                box_id = check;
                let lens_box = &mut self.boxes[box_id];
                let pos = lens_box.iter().position(|lens| lens.label == label);
                if pos.is_some() {
                    lens_box.remove(pos.unwrap());
                }
            } else if ch == '=' {
                box_id = check;
                box_pos = self.boxes[box_id].iter().position(|lens| lens.label == label);
            } else if ch.is_numeric() {
                // there is no number in the labels
                let lens = ch.to_digit(10).unwrap();
                if box_pos.is_none() {
                    self.boxes[box_id].push(Lens {
                        label: label.to_string(),
                        focal_length: lens as u8,
                    });
                } else {
                    self.boxes[box_id][box_pos.unwrap()].focal_length = lens as u8;
                }
            } else {
                label.push(ch);
            }
            check = hash(check, ch);
        }
        check
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        for op in line.split(",") {
            self.part1 += self.execute(op);
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Total hash: {}", self.part1);

        // part 2 scoring
        let part2 = (0..256).map(|box_id|
            self.boxes[box_id].iter().enumerate()
                .map(| (pos, lens)| (box_id + 1) * (pos + 1) * lens.focal_length as usize)
                .sum::<usize>()
        ).sum::<usize>();
        info!("[2] Total focusing power: {}", part2);

        Some((self.part1.to_string(), part2.to_string()))
    }
}

fn hash(val: usize, ch: char) -> usize {
    ((val + ch as usize) * 17) % 256
}

struct Lens {
    label: String,
    focal_length: u8,
}