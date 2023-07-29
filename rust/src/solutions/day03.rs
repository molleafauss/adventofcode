use std::collections::HashSet;
use crate::Solver;

pub struct Solution {
    part1: u32,
    part2: u32,
    badges: HashSet<char>,
    row: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            part1: 0,
            part2: 0,
            // the possible items are 52
            badges: HashSet::with_capacity(52),
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
                self.part1 += value(ch.into());
                break;
            }
        }

        // which elf of the triplet we seeing?
        let elf = self.row % 3;
        if elf == 1 {
            // first elf - clear current possible badges and add all letters from current line
            line.chars().for_each(|ch| { self.badges.insert(ch); });
        } else if elf == 2 {
            // remove from self.badges all chars which are not in the current line
            let not_found : Vec<char> = self.badges.iter().copied()
                .filter(|ch| line.find(*ch).is_none())
                .collect();
            not_found.iter().for_each(|ch| { self.badges.remove(ch); });
        } else {
            // remove from self.badges all chars which are not in the current line
            let not_found : Vec<char> = self.badges.iter().copied()
                .filter(|ch| line.find(*ch).is_none())
                .collect();
            not_found.iter().for_each(|ch| { self.badges.remove(ch); });
            // this should contain 1 item
            // this should run only once and we should be left with an empty (reussable) HashSet
            self.badges.drain().for_each(|ch| self.part2 += value(ch.into()));
        }
    }

    fn solve(&mut self) {
        println!("[1] Priority of item in both compartments {}", self.part1);
        println!("[2] Overall priority of badges {}", self.part2);
    }
}