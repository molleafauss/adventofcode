// What did I learn?
// very simple, discovered String::from_iterator(Iter<char>)

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::Solver;

pub(crate) struct Solution {
    fuel: i64,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            fuel: 0,
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let val = snafu_to_int(line);
        self.fuel += val;
        println!("{line} => {val} = {}", self.fuel);
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let fuel_base5 = int_to_snafu(self.fuel);
        println!("[1] {} => to base 5 {fuel_base5}", self.fuel);
        Some((fuel_base5, String::new()))
    }
}

fn snafu_to_int(text: &str) -> i64 {
    let mut val = 0;
    let mut sz = text.len() as u32;
    for ch in text.chars() {
        sz -= 1;
        let v = &VALS[&ch];
        val += v * 5_i64.pow(sz);
    }
    val
}

fn int_to_snafu(num: i64) -> String {
    let mut result = Vec::new();
    let mut val = num;
    while val > 0 {
        let rest = val % 5;
        // this should be an integer division
        val = (val - rest) / 5;
        let (mut d1, mut d0) = DIGITS[rest as usize];
        if result.len() == 0 {
            result.push(d1);
            result.push(d0);
            continue;
        }
        // add d0 to the most significant digit and change result. if a carry exist, increment d1
        d0 += result[0];
        if d0 > 2 {
            d0 -= 5;
            d1 += 1;
        }
        result[0] = d0;
        result.insert(0, d1);
    }
    while result[0] == 0 {
        result.remove(0);
    }
    // char mapping
    let mut inv_val = ['0'; 5];
    VALS.iter().for_each(|(k, v)| inv_val[(v + 2) as usize] = *k);
    String::from_iter(result.iter().map(|v| inv_val[(v + 2) as usize]))
}

static VALS: Lazy<HashMap<char, i64>> = Lazy::new(|| HashMap::from([
    ('2', 2),
    ('1', 1),
    ('0', 0),
    ('-', -1),
    ('=', -2)
]));
static DIGITS: [(i64, i64); 5] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (1, -2),
    (1, -1)
];
