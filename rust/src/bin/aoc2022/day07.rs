// What did I learn?
// I was planning to replicate 1:1 the python structure, but a using a dict like in python indexed
// on the _reference_ to the name of the entry, apparently is not a good thing (ie map[entry.name] = entry
// is bad. Unsurprisingly, as you need to hold a reference alive and the value isn't guaranteed to
// be the owning struct of the key. To implement in a similar way, someone on the Rust Discord suggested
// using an HashSet and implementing hash only for the name.
// But this implementation is actually simpler.

use std::str::FromStr;
use log::{debug, info};
use adventofcode::Solver;

pub(crate) struct Solution {
    part1: usize,
    dir_found: u32,
    dirstack: Vec<Directory>,
    alldirs: Vec<Directory>,
}

struct Directory {
    _name: String,
    size: usize,
}

impl Directory {
    fn dir(name: &str) -> Directory {
        Directory {
            _name: String::from(name),
            size: 0
        }
    }
}

const ROOT: &str = "/";
const LIMIT: usize = 100000;
const DISK_SIZE: usize = 70000000;
const MIN_FREE: usize = 30000000;

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            part1: 0,
            dir_found: 0,
            dirstack: Vec::new(),
            alldirs: Vec::new(),
        }
    }

    fn handle_command(&mut self, command: &str) {
        if command == "cd /" {
            assert_eq!(self.dirstack.len(), 0, "Dirstack is not empty! {}", self.dirstack.len());
            self.dirstack.push(Directory::dir(ROOT));
        } else if command.starts_with("cd ..") {
            let dir = self.dirstack.pop().unwrap();
            if dir.size <= LIMIT {
                self.part1 += dir.size;
            }
            self.dirstack.last_mut().unwrap().size += dir.size.clone();
            self.alldirs.push(dir);
        } else if command.starts_with("cd ") {
            // push dir "X" in dirstack
            self.dir_found += 1;
            self.dirstack.push(Directory::dir(&command[3..]));
        } else if command.starts_with("ls") {
            // nothing to do, will process entries in record_entry
        } else {
            panic!("Unknown command: {}", command);
        }
    }

    fn record_entry(&mut self, entry: &str) {
        let (size, _name) = entry.split_once(" ").unwrap();
        let curdir = self.dirstack.last_mut().unwrap();
        // ignore non-dirs for now
        if size != "dir" {
            curdir.size += usize::from_str(size).unwrap();
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.starts_with("$ ") {
            self.handle_command(&line[2..]);
        } else {
            self.record_entry(line);
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Found small dir sizes: {}", self.part1);
        while self.dirstack.len() > 1 {
            // pop all remaining dirs and save them
            let dir = self.dirstack.pop().unwrap();
            self.dirstack.last_mut().unwrap().size += dir.size;
            self.alldirs.push(dir);
        }
        // now verify we saw them all
        assert_eq!(self.alldirs.len(), self.dir_found as usize, "Missing dirs?");
        let used = self.dirstack.last().unwrap().size;
        debug!("Found size for root: {used}");
        if DISK_SIZE - used > MIN_FREE {
            info!("[2] enough space free: used {used} / free {}", DISK_SIZE - used);
            return Some((self.part1.to_string(), 0.to_string()));
        }
        let size_to_free = MIN_FREE - (DISK_SIZE - used);
        let mut big_dirs = Vec::new();
        while self.alldirs.len() > 0 {
            let dir = self.alldirs.pop().unwrap();
            if dir.size > size_to_free {
                big_dirs.push(dir);
            }
        }
        big_dirs.sort_by_key(|dir| dir.size);
        let to_delete = big_dirs.first().unwrap();
        info!("[2] min space to delete = {}", to_delete.size);
        Some((self.part1.to_string(), to_delete.size.to_string()))
    }
}

