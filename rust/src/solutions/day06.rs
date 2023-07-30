// What did I learn?
// contains, some slice/sub-slce manipulation. Used directly u8 instead of chars for simplicity.

use crate::Solver;

pub(crate) struct Solution {}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {}
    }
}

fn all_different(buf: &[u8], start: usize, length: usize) -> bool {
    let end = start + length;
    for i in start..end - 1 {
        let x = buf.get(i).unwrap();
        let found = buf[i + 1..end].contains(x);
        if found {
            return false;
        }
    }
    return true;
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let mut start_of_packet = false;
        let mut start_of_message = false;
        let mut i: usize = 0;
        while i + 14 <= line.len() {
            let bytes = line.as_bytes();
            if !start_of_packet && all_different(bytes, i, 4) {
                start_of_packet = true;
                println!("[1] Found start of packet at {}", i + 4);
            }
            if !start_of_message && all_different(bytes, i, 14) {
                start_of_message = true;
                println!("[2] Found start of message at {}", i + 14);
            }
            if start_of_packet && start_of_message {
                return;
            }
            i += 1;
        }
    }

    fn solve(&mut self) {
    }
}
