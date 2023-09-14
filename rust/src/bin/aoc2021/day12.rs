// https://adventofcode.com/2021/day/1

use std::collections::{HashMap, VecDeque};
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    start: usize,
    end: usize,
    caves: Vec<Cave>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            start: 0,
            end: 0,
            caves: Vec::new()
        }
    }

    fn find_paths(&self, allow_revisit: bool) -> usize {
        let mut explore = VecDeque::new();
        explore.push_back(vec![self.start]);
        let mut paths = Vec::new();
        while !explore.is_empty() {
            let path = explore.pop_front().unwrap();
            for conn in &self.caves[*path.last().unwrap()].connections {
                if *conn == self.start {
                    // ignore looping back on start
                    continue;
                }
                if !self.caves[*conn].large && self.cannot_visit(&path, conn, allow_revisit) {
                    // debug!("path {:?} => {} - cannot visit", path, conn);
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(*conn);
                if *conn == self.end && !paths.contains(&new_path) {
                    // end immediately pushes path if not already visited
                    // debug!("Found path {:?} - remaining {}", new_path, explore.len());
                    paths.push(new_path);
                } else if *conn != self.end {
                    explore.push_back(new_path);
                }
            }
        }
        paths.len()
    }

    fn cannot_visit(&self, path: &Vec<usize>, cave: &usize, allow_revisit: bool) -> bool {
        // if this cave is not in path -> ok
        if !path.contains(cave) {
            return false;
        }
        // if I cannot allow revisit -> short circuit
        if !allow_revisit {
            return true;
        }
        // check maximum count of caves in path. If any is present already twice -> no.
        path.iter()
            .filter(|c| !self.caves[**c].large)
            .fold(HashMap::new(),
                  |mut accum , p| {
                      accum.entry(p).and_modify(|val| *val += 1).or_insert(1);
                      accum
                  })
            .drain()
            .filter(|(_k, v)| *v >= 2 )
            .count() > 0
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        let (start, end) = line.split_once("-").unwrap();
        let mut start_id = self.caves.iter().position(|c| c.name == start);
        let mut end_id = self.caves.iter().position(|c| c.name == end);
        if start_id.is_none() {
            let id = self.caves.len();
            self.caves.push(Cave::new(id, start));
            debug!("Added cave {id} for {start} (large: {})", self.caves[id].large);
            if start == "start" {
                self.start = id
            }
            if start == "end" {
                self.end = id
            }
            start_id.insert(id);
        }
        if end_id.is_none() {
            let id = self.caves.len();
            self.caves.push(Cave::new(id, end));
            debug!("Added cave {id} for {end} (large: {})", self.caves[id].large);
            if end == "end" {
                self.end = id
            }
            if end == "start" {
                self.start = id
            }
            end_id.insert(id);
        }
        let start_id = start_id.unwrap();
        let end_id = end_id.unwrap();
        self.caves[start_id].connections.push(end_id);
        self.caves[end_id].connections.push(start_id);
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("Found {} caves; start {}, end {}", self.caves.len(), self.start, self.end);
        let part1 = self.find_paths(false);
        info!("[1] Found {} possible paths", part1);

        let part2 = self.find_paths(true);
        info!("[2] Found {} possible paths", part2);

        Some((part1.to_string(), part2.to_string()))
    }
}

struct Cave {
    id: usize,
    name: String,
    large: bool,
    // connect by id into the caves vector
    connections: Vec<usize>,
}

impl Cave {
    fn new(id: usize, name: &str) -> Cave {
        Cave {
            id,
            name: String::from(name),
            large: name.chars().all(|ch| ch.is_ascii_uppercase()),
            connections: Vec::new(),
        }
    }
}