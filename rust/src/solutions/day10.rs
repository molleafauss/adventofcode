// What did I learn?
// Vec implements Index (it's slices that do not) - :facepalm: retconned all previous exercises.
// str::from_utf8 works on slices, String::from_utf8 works on Vec.

use std::str::FromStr;
use std::str;
use log::{debug, info};
use crate::Solver;

pub(crate) struct Solution {
    x: i32,
    cpos: usize,
    cycle: i32,
    signal_strength: i32,
    row: usize,
    col: usize,
    display: Vec<u8>,
}

const CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];
const COLS: usize = 40;
const ROWS: usize = 6;
const EMPTY: u8 = ' ' as u8;
const DOT: u8 = '.' as u8;
const POUND: u8 = '#' as u8;

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            x: 1,
            cpos: 0,
            cycle: 0,
            signal_strength: 0,
            row: 0,
            col: 0,
            display: vec![EMPTY; ROWS * COLS],
        }
    }

    fn check_cycle(&mut self, ticks: i32) {
        if self.cpos >= CYCLES.len() {
            return;
        }
        let cycle_val = CYCLES.get(self.cpos).unwrap();
        if self.cycle + ticks >= *cycle_val {
            let s = self.x * cycle_val;
            self.signal_strength += s;
            debug!("Signal strength at cycle {}/{ticks}: {s} => {}", self.cycle, self.signal_strength);
            self.cpos += 1;
        }
    }

    fn draw(&mut self) {
        let draw = if self.x - 1 <= self.col as i32 && self.col as i32 <= self.x + 1 { POUND } else { DOT };
        let pos = self.row * COLS + self.col;
        self.display[pos] = draw;
        self.col += 1;
        if self.col >= COLS {
            self.col = 0;
            self.row += 1;
        }
        if self.row >= ROWS {
            self.show_display();
            self.row = 0;
        }
    }

    fn show_display(&self) {
        (0..ROWS).for_each(|row| {
            println!("{}", str::from_utf8(&self.display[row * COLS..(row+1) * COLS]).unwrap());
        });
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.starts_with("noop") {
            self.draw();
            self.check_cycle(1);
            self.cycle += 1;
        } else if line.starts_with("addx") {
            self.draw();
            self.draw();
            self.check_cycle(2);
            self.x += i32::from_str(&line[5..]).unwrap();
            self.cycle += 2;
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Signal strength found: {}",self.signal_strength);
        Some((self.signal_strength.to_string(), String::new()))
    }
}
