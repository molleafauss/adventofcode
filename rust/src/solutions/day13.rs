use std::cmp::Ordering;
use crate::Solver;

pub(crate) struct Solution {
    right_order: usize,
    packets: Vec<Packet>,
    pairs: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            right_order: 0,
            packets: Vec::new(),
            pairs: 0
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            if self.packets[self.pairs * 2] < self.packets[self.pairs * 2 + 1] {
                println!("pairs {} - right order", self.pairs);
                self.right_order += self.pairs + 1;
            }
            self.pairs += 1;
        } else {
            self.packets.push(Packet::parse(line));
        }
    }

    fn solve(&mut self) {
        println!("[1] Right order value: {}", self.right_order);

        let mut decoder_key = 1;
        let mut divider_packets = 0;
        let divider_1 = Packet::parse("[[2]]");
        self.packets.push(divider_1.clone());
        let divider_2 = Packet::parse("[[6]]");
        self.packets.push(divider_2.clone());
        self.packets.sort();
        self.packets.iter().enumerate().for_each(|(i, packet)| {
            if packet == &divider_1 || packet == &divider_2 {
                println!("Found divider at position {i}: {:?}", packet);
                divider_packets += 1;
                decoder_key *= i + 1;
            }
        });
        assert_eq!(divider_packets, 2, "Did not find all divider packets");
        println!("[2] decoder key: {decoder_key}");
    }
}

#[derive(Eq, Clone, Debug)]
enum Packet {
    List(Vec<Packet>),
    Value(u32),
}

impl Packet {
    fn parse(line: &str) -> Packet {
        println!("Parsing {}", line);
        assert!(line.starts_with("[") && line.ends_with("]"), "Line not a list? {line}");
        // packet is always at least an empty list
        let mut stack = vec![Packet::List(Vec::new())];
        // skip start and end and iterate on chars
        line[1..line.len() - 1].chars().enumerate().for_each(|(i, ch)| {
            if ch == '[' {
                // create a new list, add it to the stack
                stack.push(Packet::List(Vec::new()));
                // println!("Found [ => {}", stack.len());
            } else if ch == ']' {
                // pop the last element (should be a list) and add it to the parent
                let el = stack.pop().unwrap();
                let Packet::List(prev) = stack.last_mut().unwrap() else {
                    panic!("Unable to push items in Packet - not a List? At char {i}");
                };
                prev.push(el);
                // println!("Found ] => {}", stack.len());
            } else if ch == ',' {
                let value = stack.pop().unwrap();
                // println!("Found , => pushing {:?} into stack {}", value, stack.len());
                let Packet::List(prev) = stack.last_mut().unwrap() else {
                    panic!("Didn't find a list to insert into? At char {i}");
                };
                prev.push(value);
            } else if ch.is_ascii_digit() {
                // if last is a list - we create a new value, if it's a value
                let last = stack.last_mut().unwrap();
                if let Packet::Value(value) = last {
                    *value *= 10;
                    *value += ch.to_digit(10).unwrap();
                    // println!("Found digit, new value {}", value);
                } else {
                    stack.push(Packet::Value(ch.to_digit(10).unwrap()));
                    // println!("Found digit, added new value {}", ch);
                }
            }
        });

        if stack.len() == 2 {
            let value = stack.pop().unwrap();
            let Packet::List(head) = stack.last_mut().unwrap() else {
                panic!("Head is not a list?");
            };
            head.push(value);
        } else if stack.len() != 1 {
            panic!("List not parsed correctly - remaining items: {}", stack.len());
        }

        stack.pop().unwrap()
    }

    fn as_list(val: u32) -> Packet {
        Packet::List(vec![Packet::Value(val)])
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let Packet::List(left) = self else {
            panic!("Self is not a list?");
        };
        let Packet::List(right) = other else {
            panic!("Other is not a list?");
        };
        for i in 0..left.len() {
            if i >= right.len() {
                return Ordering::Greater;
            }
            let lval = &left[i];
            let rval = &right[i];
            // one of the values is a list, recurse
            let result = match (lval, rval) {
                (Packet::List(_), Packet::Value(rv)) => lval.cmp(&Packet::as_list(*rv)),
                (Packet::Value(lv), Packet::List(_)) => Packet::as_list(*lv).cmp(rval),
                (Packet::List(_), Packet::List(_)) => lval.cmp(rval),
                (Packet::Value(lv), Packet::Value(rv)) => {
                    if lv == rv {
                        Ordering::Equal
                    } else if lv < rv {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            };
            if result != Ordering::Equal {
                return result;
            }
        }
        if left.len() == right.len() { Ordering::Equal } else { Ordering::Less }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
