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
use log::{info};

pub(crate) struct Solution {
    valves: HashMap<String, Valve>,
    connections: HashMap<String, Vec<String>>,
    valves_with_flow: u32,
}

static RE_VALVE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap());
const PART1_MINUTES: i32 = 30;
const PART2_MINUTES: i32 = 26;
const START: &str = "AA";

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            valves: HashMap::new(),
            connections: HashMap::new(),
            valves_with_flow: 0,
        }
    }
    
    fn find_valves_with_flow(&mut self) -> Vec<Valve> {
        // create the vector of the valve we are interested in: start + those with flow
        let mut valves_with_flow = Vec::new();
        valves_with_flow.push(self.valves[START].clone());
        self.valves.iter().for_each(|(_, valve)| {
            if valve.flow > 0 {
                valves_with_flow.push(valve.clone());
            }
        });
        valves_with_flow
    }
    
    fn calculate_distances(&self, valves_with_flow: &mut Vec<Valve>) {
        let mut all_distances = Vec::new();
        let valves_length = valves_with_flow.len();
        valves_with_flow.iter().for_each(|curr| {
            let mut visited = HashSet::from([&curr.name]);
            let mut queue = vec![(&curr.name, 0)];
            let mut distances = vec![0; valves_length];
            while !queue.is_empty() {
                let (name, distance) = queue.remove(0);
                self.connections[name].iter().for_each(|next| {
                    if visited.contains(&next) {
                        return;
                    }
                    visited.insert(&next);
                    let id = self.valves[next].id;
                    if next == START || id != 0 {
                        distances[id as usize] = distance + 1;
                    }
                    queue.push((next, distance + 1));
                });
            }
            all_distances.push(distances);
        });
        
        valves_with_flow.iter_mut().for_each(|v| v.tunnels = all_distances[v.id as usize].clone());
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if let Some(captures) = RE_VALVE.captures(line) {
            let valve = Valve::new(&captures, self.valves_with_flow);
            if valve.flow > 0 {
                self.valves_with_flow += 1;
            }
            self.connections.insert(valve.name.clone(), captures[3].split(", ")
                .map(|part| String::from(part))
                .collect());
            self.valves.insert(valve.name.clone(), valve);
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("Found {} valves to open in {PART1_MINUTES} minutes", self.valves.len());
        let mut valves_with_flow = self.find_valves_with_flow();
        self.calculate_distances(&mut valves_with_flow);
        info!("Valves with flow: {}", valves_with_flow.len());

        // part 1 - timed
        let t0 = SystemTime::now();
        let mut one_path = OnePathSolver::new(&valves_with_flow);
        let best_path1 = one_path.find_path(&self, OnePath::new(START));
        let t1 = SystemTime::now();
        info!("[1] Found max flow is {}: {:?} ({} cache hits, {} calls, {} cache size) [{:.3}sec]",
                 best_path1.total_flow, best_path1.visited, one_path.cache_hits, one_path.calls,
                 one_path.cache.len(), t1.duration_since(t0).unwrap().as_secs_f32());

        // part 2
        let t0 = SystemTime::now();
        let mut two_path = TwoPathsSolver::new(&valves_with_flow);
        let best_path2 = two_path.find_path(&self, TwoPaths::new(START));
        let t1 = SystemTime::now();
        info!("[2] Found max flow is {}: {:?} / {:?} ({} cache hits, {} calls, {} cache size) [{:.3}sec]",
                 best_path2.total_flow, best_path2.human_path, best_path2.ele_path, two_path.cache_hits,
                 two_path.calls, two_path.cache.len(), t1.duration_since(t0).unwrap().as_secs_f32());
        Some((best_path1.total_flow.to_string(), best_path2.total_flow.to_string()))
    }
}

#[derive(Clone)]
struct Valve {
    id: u8,
    tunnels: Vec<u8>,
    name: String,
    flow: u8,
    mask: u32,
}

impl Valve {
    fn new(captures: &Captures, valves_with_flow: u32) -> Valve {
        let flow = u8::from_str(&captures[2]).unwrap();
        Valve {
            id: if flow > 0 { (valves_with_flow + 1) as u8 } else { 0 },
            tunnels: Vec::new(),
            name: String::from(&captures[1]),
            flow,
            mask: 1 << (valves_with_flow),
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

    fn next(&self, valve: &Valve, distance: u8) -> OnePath {
        let mut visited = self.visited.clone();
        visited.push(valve.name.clone());
        let elapsed = self.elapsed + distance as i32 + 1;
        let flow = (PART1_MINUTES - elapsed) * valve.flow as i32;
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

struct OnePathSolver<'a> {
    calls: u32,
    cache: HashMap<OnePathKey, OnePath>,
    cache_hits: u32,
    valves_with_flow: &'a Vec<Valve>,
}

impl OnePathSolver<'_> {
    fn new(valves_with_flow: &Vec<Valve>) -> OnePathSolver {
        OnePathSolver {
            calls: 0,
            cache: HashMap::new(),
            cache_hits: 0,
            valves_with_flow,
        }
    }

    fn find_path(&mut self, data: &Solution, path: OnePath) -> OnePath {
        self.calls += 1;
        if (self.calls % 1000000) == 0 {
            info!("{} calls, {} cache hits...", self.calls, self.cache_hits)
        }
        let cache_key = path.cache_key();
        if self.cache.contains_key(&cache_key) {
            self.cache_hits += 1;
            let cached = &self.cache[&cache_key];
            return path.merge(cached);
        }

        let cave = path.visited.last().unwrap();
        let curr_valve = &data.valves[cave];
        let mut best_path = path.clone();
        self.valves_with_flow.iter().for_each(|valve| {
            if path.open_valves & valve.mask != 0 {
                return ;
            }
            let distance = valve.tunnels[curr_valve.id as usize];
            let next = path.next(valve, distance);
            if next.elapsed >= PART1_MINUTES {
                return ;
            }
            let sub_best = self.find_path(data, next);
            if sub_best.total_flow > best_path.total_flow {
                best_path = sub_best;
            }
        });

        self.cache.insert(cache_key, best_path.diff(&path));
        best_path
    }
}

// --- part 2 ---

#[derive(Hash, Eq, PartialEq)]
struct TwoPathsKey(String, u8, String, u8, u32);

#[derive(Clone)]
struct TwoPaths {
    human_path: Vec<String>,
    human_elapsed: u8,
    ele_path: Vec<String>,
    ele_elapsed: u8,
    open_valves: u32,
    elapsed: u8,
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

    fn next_human(&self, valve: &Valve, distance: u8) -> TwoPaths {
        let mut human_path = self.human_path.clone();
        human_path.push(valve.name.clone());
        let ele_path = self.ele_path.clone();
        let elapsed = self.human_elapsed + distance + 1;
        let flow = (PART2_MINUTES - elapsed as i32) * valve.flow as i32;
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

    fn next_elephant(&self, valve: &Valve, distance: u8) -> TwoPaths {
        let human_path = self.human_path.clone();
        let mut ele_path = self.ele_path.clone();
        ele_path.push(valve.name.clone());
        let elapsed = self.ele_elapsed + distance + 1;
        let flow = (PART2_MINUTES - elapsed as i32) * valve.flow as i32;
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

struct TwoPathsSolver<'a> {
    calls: u32,
    cache: HashMap<TwoPathsKey, TwoPaths>,
    cache_hits: u32,
    valves_with_flow: &'a Vec<Valve>
}

impl TwoPathsSolver<'_> {
    fn new(valves_with_flow: &Vec<Valve>) -> TwoPathsSolver {
        TwoPathsSolver {
            calls: 0,
            cache: HashMap::new(),
            cache_hits: 0,
            valves_with_flow,
        }
    }

    fn find_path(&mut self, data: &Solution, path: TwoPaths) -> TwoPaths {
        self.calls += 1;
        if (self.calls % 1000000) == 0 {
            info!("{} calls, {} cache hits...", self.calls, self.cache_hits)
        }

        let cache_key = path.cache_key();
        if self.cache.contains_key(&cache_key) {
            self.cache_hits += 1;
            let cached = &self.cache[&cache_key];
            return path.merge(cached);
        }

        let man_pos = path.human_path.last().unwrap();
        let man_valve = &data.valves[man_pos];
        let ele_pos = path.ele_path.last().unwrap();
        let ele_valve = &data.valves[ele_pos];

        let mut best_path = path.clone();
        self.valves_with_flow.iter().for_each(|valve| {
            // try to move both human and elephant towards the next valve
            if path.open_valves & valve.mask != 0 {
                return ;
            }
            // move human
            let distance = valve.tunnels[man_valve.id as usize];
            let next = path.next_human(valve, distance);
            if next.elapsed < PART2_MINUTES as u8 {
                let sub_best = self.find_path(data, next);
                if sub_best.total_flow > best_path.total_flow {
                    best_path = sub_best;
                }
            }

            // move elephant
            let distance = valve.tunnels[ele_valve.id as usize];
            let next = path.next_elephant(valve, distance);
            if next.elapsed < PART2_MINUTES as u8 {
                let sub_best = self.find_path(data, next);
                if sub_best.total_flow > best_path.total_flow {
                    best_path = sub_best;
                }
            }
        });

        self.cache.insert(cache_key, best_path.diff(&path));
        best_path
    }
}
