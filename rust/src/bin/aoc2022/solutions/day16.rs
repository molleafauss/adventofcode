// What did I learn?
// using a Vec instead of a HashSet for storing the valves, considering the valves are "few", is
// O(N) on string search faster than calculating a hash - need to see difference (maybe store ids/position
// in vector)
// Had to split the implementation in two structs as the cache_key is different.
// Part 2 time - 9min vs 30ish (python)
// TODO - Possible optimization: index valves/distances into a Vec by position, search always
// directly by index (should save some time)
// apparently is slower than python?

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::SystemTime;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use adventofcode::Solver;
use factorial::Factorial;
use log::{debug, info};

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

    fn solve(&mut self) -> Option<(String, String)> {
        debug!("Found {} valves to open in {PART1_MINUTES} minutes", self.valves.len());
        debug!("Valves with flow: {} => {} possible paths",
                 self.valves_with_flow.len(), self.valves_with_flow.len().factorial());
        self.calculate_distances();

        // part 1 - timed
        let t0 = SystemTime::now();
        let mut one_path = OnePathSolver::new();
        let best_path1 = one_path.find_path(&self, OnePath::new(START));
        let t1 = SystemTime::now();
        info!("[1] Found max flow is {}: {:?} ({} cache hits) [{:.3}sec]",
                 best_path1.total_flow, best_path1.visited, one_path.cache_hits, t1.duration_since(t0).unwrap().as_secs_f32());

        // part 2
        let t0 = SystemTime::now();
        let mut two_path = TwoPathsSolver::new();
        let best_path2 = two_path.find_path(&self, TwoPaths::new(START));
        let t1 = SystemTime::now();
        info!("[2] Found max flow is {}: {:?} / {:?} ({} cache hits) [{:.3}sec]",
                 best_path2.total_flow, best_path2.human_path, best_path2.ele_path, two_path.cache_hits,
                 t1.duration_since(t0).unwrap().as_secs_f32());
        Some((best_path1.total_flow.to_string(), best_path2.total_flow.to_string()))
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

// --- part 1 ---

#[derive(Hash, Eq, PartialEq)]
struct OnePathKey(String, i32, u32);

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
        OnePathKey(String::from(cave), self.elapsed, self.open_valves)
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
    calls: u32,
    cache: HashMap<OnePathKey, OnePath>,
    cache_hits: u32,
}

impl OnePathSolver {
    fn new() -> OnePathSolver {
        OnePathSolver {
            calls: 0,
            cache: HashMap::new(),
            cache_hits: 0,
        }
    }

    fn find_path(&mut self, data: &Solution, path: OnePath) -> OnePath {
        self.calls += 1;
        if (self.calls % 1000000) == 0 {
            println!("{} calls, {} cache hits...", self.calls, self.cache_hits)
        }
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

// --- part 2 ---

#[derive(Hash, Eq, PartialEq)]
struct TwoPathsKey(String, i32, String, i32, u32);

#[derive(Clone)]
struct TwoPaths {
    human_path: Vec<String>,
    human_elapsed: i32,
    ele_path: Vec<String>,
    ele_elapsed: i32,
    open_valves: u32,
    elapsed: i32,
    total_flow: i32,
}

impl TwoPaths {
    fn new(start: &str) -> TwoPaths {
        TwoPaths {
            human_path: vec![String::from(start)],
            human_elapsed: 0,
            ele_path: vec![String::from(start)],
            ele_elapsed: 0,
            open_valves: 0,
            elapsed: 0,
            total_flow: 0,
        }
    }

    fn cache_key(&self) -> TwoPathsKey {
        TwoPathsKey(
            String::from(self.human_path.last().unwrap()),
            self.human_elapsed,
            String::from(self.ele_path.last().unwrap()),
            self.ele_elapsed,
            self.open_valves,
        )
    }

    fn merge(&self, other: &TwoPaths) -> TwoPaths {
        let mut human_path = self.human_path.clone();
        human_path.extend_from_slice(&other.human_path[..]);
        let mut ele_path = self.ele_path.clone();
        ele_path.extend_from_slice(&other.ele_path[..]);
        TwoPaths {
            human_path,
            human_elapsed: self.human_elapsed + other.human_elapsed,
            ele_path,
            ele_elapsed: self.ele_elapsed + other.ele_elapsed,
            open_valves: self.open_valves,
            elapsed: self.elapsed + other.elapsed,
            total_flow: self.total_flow + other.total_flow,
        }
    }

    fn next_human(&self, valve: &Valve, distance: i32) -> TwoPaths {
        let mut human_path = self.human_path.clone();
        human_path.push(valve.name.clone());
        let ele_path = self.ele_path.clone();
        let elapsed = self.human_elapsed + distance + 1;
        let flow = (PART2_MINUTES - elapsed) * valve.flow;
        TwoPaths {
            human_path,
            human_elapsed: elapsed,
            ele_path,
            ele_elapsed: self.ele_elapsed,
            open_valves: self.open_valves | valve.mask,
            elapsed: max(elapsed, self.ele_elapsed),
            total_flow: self.total_flow + flow
        }
    }

    fn next_elephant(&self, valve: &Valve, distance: i32) -> TwoPaths {
        let human_path = self.human_path.clone();
        let mut ele_path = self.ele_path.clone();
        ele_path.push(valve.name.clone());
        let elapsed = self.ele_elapsed + distance + 1;
        let flow = (PART2_MINUTES - elapsed) * valve.flow;
        TwoPaths {
            human_path,
            human_elapsed: self.human_elapsed,
            ele_path,
            ele_elapsed: elapsed,
            open_valves: self.open_valves | valve.mask,
            elapsed: max(elapsed, self.human_elapsed),
            total_flow: self.total_flow + flow
        }
    }

    fn diff(&self, start: &TwoPaths) -> TwoPaths {
        let human_path = self.human_path.split_at(start.human_path.len()).1.iter().cloned().collect();
        let ele_path = self.ele_path.split_at(start.ele_path.len()).1.iter().cloned().collect();
        TwoPaths {
            human_path,
            human_elapsed: self.human_elapsed - start.human_elapsed,
            ele_path,
            ele_elapsed: self.ele_elapsed - start.ele_elapsed,
            open_valves: self.open_valves,
            elapsed: self.elapsed - start.elapsed,
            total_flow: self.total_flow - start.total_flow,
        }
    }
}

struct TwoPathsSolver {
    calls: u32,
    cache: HashMap<TwoPathsKey, TwoPaths>,
    cache_hits: u32,
}

impl TwoPathsSolver {
    fn new() -> TwoPathsSolver {
        TwoPathsSolver {
            calls: 0,
            cache: HashMap::new(),
            cache_hits: 0,
        }
    }

    fn find_path(&mut self, data: &Solution, path: TwoPaths) -> TwoPaths {
        self.calls += 1;
        if (self.calls % 1000000) == 0 {
            println!("{} calls, {} cache hits...", self.calls, self.cache_hits)
        }
        let man_pos = path.human_path.last().unwrap();
        let ele_pos = path.ele_path.last().unwrap();
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
            // try to move both human and elephant towards the next valve
            if path.open_valves & valve.mask != 0 {
                continue;
            }
            // move human
            let distance = data.distances[man_pos][name];
            let next = path.next_human(valve, distance);
            if next.elapsed < PART2_MINUTES {
                let sub_best = self.find_path(data, next);
                if sub_best.total_flow > best_path.total_flow {
                    best_path = sub_best;
                }
            }

            // move elephant
            let distance = data.distances[ele_pos][name];
            let next = path.next_elephant(valve, distance);
            if next.elapsed < PART2_MINUTES {
                let sub_best = self.find_path(data, next);
                if sub_best.total_flow > best_path.total_flow {
                    best_path = sub_best;
                }
            }
        }

        self.cache.insert(cache_key, best_path.diff(&path));
        return best_path;
    }
}
