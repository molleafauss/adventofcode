use crate::Solver;

pub struct Solution {
    part1: u32,
    part2: u32,
    // this contains one slot for each possible item, and then
    badges: Vec<u8>,
    row: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            part1: 0,
            part2: 0,
            badges: vec![0, 52],
            row: 0,
        }
    }
}

fn value(ch: u32) -> u32 {
    let v = ch;
    if char::from_u32(v).unwrap().is_ascii_lowercase() {
        return ch - ('a' as u32) + 1;
    } else {
        return ch - ('A' as u32) + 27;
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        self.row += 1;
        assert_eq!(line.len() % 2, 0, "Line {} has not even length: {}", line, line.len());
        let mid = line.len() / 2;
        let (first_half, second_half) = line.split_at(mid);
        for b in second_half.as_bytes() {
            let ch = char::from(*b);
            if first_half.find(ch) != None {
                // let i : u32 = ch.into();
                // let val = ;
                self.part1 += value(ch.into());
                break;
            }
        }
    }

    fn solve(&mut self) {
        println!("[1] Priority of item in both compartments {}", self.part1);
        println!("[2] Overall priority of badges {}", self.part2);
    }
}