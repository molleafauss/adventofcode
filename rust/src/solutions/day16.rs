// What did I learn?
// using a Vec instead of a HashSet for storing the valves, considering the valves are "few", is
// O(N) on string search faster than calculating a hash?

use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::SystemTime;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use crate::Solver;
use factorial::Factorial;

pub(crate) struct Solution {
    valves: Vec<Valve>,
    valves_with_flow: Vec<String>,
    distances: HashMap<String, HashMap<String, i32>>,
}

static RE_VALVE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap());
const PART1_MINUTES: i32 = 30;
const PART2_MINUTES: i32 = 26;
const START: &str = "AA";

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            valves: Vec::new(),
            valves_with_flow: Vec::new(),
            distances: HashMap::new(),
        }
    }

    fn calculate_distances(&mut self) {
        // add (temporarily) the start into the valves that need to be evaluated
        self.valves_with_flow.insert(0, START.into());
        for name in &self.valves_with_flow {
            let mut distances = HashMap::from([(name.clone(), 0)]);
            let mut visited = HashSet::from([name]);
            let mut queue = vec![(name, 0)];
            while !queue.is_empty() {
                let (cave, distance) = queue.remove(0);
                let valve = self.get_valve(cave);
                for next in &valve.connections {
                    if visited.contains(next) {
                        continue;
                    }
                    visited.insert(next);
                    if self.valves_with_flow.iter().find(|&v| v == next).is_some() {
                        distances.insert(next.clone(), distance + 1);
                    }
                    queue.push((next, distance + 1));
                }
            }
            distances.remove(name);
            self.distances.insert(name.clone(), distances);
        }
        self.valves_with_flow.remove(0);
    }

    fn get_valve(&self, name: &str) -> &Valve {
        self.valves.iter().find(|v| v.name == name).unwrap()
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if let Some(captures) = RE_VALVE.captures(line) {
            let mut valve = Valve::new(&captures);
            if valve.flow > 0 {
                self.valves_with_flow.push(valve.name.clone());
                valve.mask = 1 << self.valves_with_flow.len();
            }
            self.valves.push(valve);
        }
    }

    fn solve(&mut self) {
        println!("Found {} valves to open in {PART1_MINUTES} minutes", self.valves.len());
        println!("Valves with flow: {} => {} possible paths",
                 self.valves_with_flow.len(), self.valves_with_flow.len().factorial());
        self.calculate_distances();

        // part 1 - time it
        let t0 = SystemTime::now();
        let mut one_path = OnePathSolver::new();
        let best_path = one_path.find_path(&self, OnePath::new(START));
        let t1 = SystemTime::now();
        println!("[1] Found max flow is {}: {:?} ({} cache hits) [{:.3}sec]",
                 best_path.total_flow, best_path.visited, one_path.cache_hits, t1.duration_since(t0).unwrap().as_secs_f32());
    }
}

struct Valve {
    name: String,
    flow: i32,
    mask: u32,
    connections: Vec<String>,
}

impl Valve {
    fn new(captures: &Captures) -> Valve {
        let connections = captures[3].split(", ")
            .map(|part| String::from(part))
            .collect();
        Valve {
            name: String::from(&captures[1]),
            flow: i32::from_str(&captures[2]).unwrap(),
            mask: 0,
            connections,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct OnePathKey(String, i32, i32);

#[derive(Clone)]
struct OnePath {
    visited: Vec<String>,
    open_valves: u32,
    elapsed: i32,
    total_flow: i32,
}

impl OnePath {
    fn new(start: &str) -> OnePath {
        OnePath {
            visited: vec![String::from(start)],
            open_valves: 0,
            elapsed: 0,
            total_flow: 0,
        }
    }

    fn cache_key(&self) -> OnePathKey {
        let cave = self.visited.last().unwrap();
        OnePathKey(String::from(cave), self.elapsed, self.total_flow)
    }

    fn merge(&self, other: &OnePath) -> OnePath {
        let mut visited = self.visited.clone();
        visited.extend_from_slice(&other.visited[..]);
        OnePath {
            visited,
            open_valves: self.open_valves,
            elapsed: self.elapsed + other.elapsed,
            total_flow: self.total_flow + other.total_flow,
        }
    }

    fn next(&self, valve: &Valve, distance: i32) -> OnePath {
        let mut visited = self.visited.clone();
        visited.push(valve.name.clone());
        let elapsed = self.elapsed + distance + 1;
        let flow = (PART1_MINUTES - elapsed) * valve.flow;
        OnePath {
            visited,
            open_valves: self.open_valves | valve.mask,
            elapsed,
            total_flow: self.total_flow + flow
        }
    }

    fn diff(&self, start: &OnePath) -> OnePath {
        let visited = self.visited.split_at(start.visited.len()).1.iter().cloned().collect();
        OnePath {
            visited,
            open_valves: self.open_valves,
            elapsed: self.elapsed - start.elapsed,
            total_flow: self.total_flow - start.total_flow,
        }
    }
}

struct OnePathSolver {
    cache: HashMap<OnePathKey, OnePath>,
    cache_hits: u32,
}

impl OnePathSolver {
    fn new() -> OnePathSolver {
        OnePathSolver {
            cache: HashMap::new(),
            cache_hits: 0,
        }
    }

    fn find_path(&mut self, data: &Solution, path: OnePath) -> OnePath {
        let cave = path.visited.last().unwrap();
        let cache_key = path.cache_key();
        if self.cache.contains_key(&cache_key) {
            self.cache_hits += 1;
            let cached = &self.cache[&cache_key];
            return path.merge(cached);
        }

        let mut best_path = path.clone();
        for i in 0..data.valves_with_flow.len() {
            let name = &data.valves_with_flow[i];
            let valve = data.get_valve(name);
            if path.open_valves & valve.mask != 0 {
                continue;
            }
            let distance = data.distances[cave][name];
            let next = path.next(valve, distance);
            if next.elapsed >= PART1_MINUTES {
                continue;
            }
            let sub_best = self.find_path(data, next);
            if sub_best.total_flow > best_path.total_flow {
                best_path = sub_best;
            }
        }

        self.cache.insert(cache_key, best_path.diff(&path));
        return best_path;
    }
}