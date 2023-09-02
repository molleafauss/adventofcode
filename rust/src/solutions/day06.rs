// What did I learn?
// contains, some slice/sub-slce manipulation. Used directly u8 instead of chars for simplicity.

use log::info;
use crate::Solver;

pub(crate) struct Solution {
    start_of_packet: Option<usize>,
    start_of_message: Option<usize>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            start_of_packet: None,
            start_of_message: None,
        }
    }
}

fn all_different(buf: &[u8], start: usize, length: usize) -> bool {
    let end = start + length;
    for i in start..end - 1 {
        let x = &buf[i];
        let found = buf[i + 1..end].contains(x);
        if found {
            return false;
        }
    }
    return true;
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let mut i: usize = 0;
        while i + 14 <= line.len() {
            let bytes = line.as_bytes();
            if self.start_of_packet.is_none() && all_different(bytes, i, 4) {
                self.start_of_packet = Some(i + 4);
                info!("[1] Found start of packet at {}", i + 4);
            }
            if self.start_of_message.is_none() && all_different(bytes, i, 14) {
                self.start_of_message = Some(i + 14);
                info!("[2] Found start of message at {}", i + 14);
            }
            if self.start_of_packet.is_some() && self.start_of_message.is_some() {
                return;
            }
            i += 1;
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        Some((self.start_of_packet.unwrap().to_string(), self.start_of_message.unwrap().to_string()))
    }
}
