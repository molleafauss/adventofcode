// What did I learn?
// % in rust is the _remainder_ not the _modulo_ operation, so I needed a special function.
// also there is the rem_euclid function that seems to implement python-like modulo, but doesn't
// seem to give the result I expected (despite examples)?

use std::str::FromStr;
use crate::Solver;

pub(crate) struct Solution {
    data: Vec<(i64, usize)>,
    initial_order: usize,
    size: i64,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            data: Vec::new(),
            initial_order: 0,
            size: 0
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        self.data.push((i64::from_str(line).unwrap(), self.initial_order));
        self.initial_order += 1;
        self.size += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        // part 1
        let result = self.mix(self.data.clone());
        let total1 = self.coordinates(&result);
        println!("[1] Final coordinates: {total1}");

        // part 2
        let mut data = self.data.iter().map(|(v, pos)| (v * ENCRYPTION_KEY, *pos)).collect();
        for _i in 0..10 {
            data = self.mix(data);
        }
        let total2 = self.coordinates(&data);
        println!("[2] Final coordinates: {total2}");
        Some((total1.to_string(), total2.to_string()))
    }
}

const FINAL: [i64; 3] = [1000, 2000, 3000];
const ENCRYPTION_KEY: i64 = 811589153;

impl Solution {
    fn mix(&self, mut data: Vec<(i64, usize)>) -> Vec<(i64, usize)> {
        let mut original_idx = 0;
        while original_idx < self.size {
            let pos = data.iter().position(|&x| x.1 == original_idx as usize).unwrap();
            original_idx += 1;
            if data[pos].0 == 0 {
                // don't move
                continue;
            }
            let pos_moved = pos as i64 + data[pos].0;
            // wrap by one less because pop() will shorten the list
            let new_pos = wrap(pos_moved, self.size - 1);
            let val = data.remove(pos);
            // println!("Moving {} {:?} => {pos_moved} => {new_pos} [size: {}/{}]", original_idx - 1, val, self.size, data.len());
            if new_pos == 0 || new_pos == self.size - 1 {
                data.push(val);
            } else {
                data.insert(new_pos as usize, val);
            }

        }
        data
    }

    fn coordinates(&self, data: &Vec<(i64, usize)>) -> i64 {
        let zero = data.iter()
            .position(|&x| x.0 == 0).unwrap() as i64;
        let mut total = 0;
        for pos in FINAL {
            let idx = (zero + pos) % self.size;
            let val = data[idx as usize].0;
            total += val;
            println!("{pos}: {val} -> {total}");
        }
        total
    }
}

/// wraps the given val number within 0..max (max excluded).
/// if > 0 -> standard %
/// if < 0 -> get mod of
fn wrap(val: i64, max: i64) -> i64 {
    if val >= 0 {
        val % max
    } else {
        max - (val.abs() % max)
    }
}

#[cfg(test)]
mod tests {
    use super::wrap;

    #[test]
    fn wrap_tests() {
        assert_eq!(wrap(0, 6), 0);
        assert_eq!(wrap(6, 6), 0);
        assert_eq!(wrap(7, 6), 1);
        assert_eq!(wrap(66, 6), 0);
        assert_eq!(wrap(6, 6), 0);
        assert_eq!(wrap(-1, 6), 5);
        assert_eq!(wrap(-11, 6), 1);
        // These should work but they do not?
        // assert_eq!(-7_i64.rem_euclid(4_i64), 1);
        // assert_eq!(-11_i64.rem_euclid(6_i64), 1);
    }
}
