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
        }
    }

    fn find_valves_with_flow(&mut self) -> Vec<Valve> {
        // create the vector of the valve we are interested in: start and those with flow
        let mut valves_with_flow = Vec::new();
        
        for (_, valve) in &self.valves {
            if valve.flow == 0 {
                continue;
            }
            let l = valves_with_flow.len();
            let mut v = valve.clone();
            v.id = (l + 1) as u8;
            v.mask = 1 << l;
            valves_with_flow.push(v);
        }

        let mut start_valve = self.valves[START].clone();
        start_valve.id = 0;
        valves_with_flow.insert(0, start_valve);

        valves_with_flow
    }

    fn calculate_distances(&self, valves_with_flow: &mut Vec<Valve>) {
        let mut all_distances = Vec::new();
        let valves_length = valves_with_flow.len();
        let valves_id_map : HashMap<&String, usize> = valves_with_flow.iter().enumerate()
            .map(|(i, v)| (&v.name, i))
            .collect();
        valves_with_flow.iter().for_each(|curr| {
            let mut tunnels = vec![0; valves_length];
            let mut visited = HashSet::from([&curr.name]);
            let mut queue = vec![(&curr.name, 0)];
            while !queue.is_empty() {
                let (name, distance) = queue.remove(0);
                for next in &self.connections[name] {
                    if visited.contains(&next) {
                        continue;
                    }
                    visited.insert(&next);
                    valves_id_map.get(next).map(|id| tunnels[*id] = distance + 1);
                    queue.push((next, distance + 1));
                }
            }
            info!("Distances for {}: {:?}", curr.name, tunnels);
            all_distances.push(tunnels);
        });
        
        let mut idx = 0;
        for distances in all_distances {
            valves_with_flow[idx].tunnels = distances;
            idx += 1;
        }
        // valves_with_flow.iter_mut().for_each(|v| v.tunnels = all_distances[v.id as usize].clone
        // ());
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if let Some(captures) = RE_VALVE.captures(line) {
            let valve = Valve::new(&captures);
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
        let best_path2 = two_path.find_path(&self, TwoPaths::new(valves_with_flow.len()));
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
    fn new(captures: &Captures) -> Valve {
        let flow = u8::from_str(&captures[2]).unwrap();
        Valve {
            id: 0,
            tunnels: Vec::new(),
            name: String::from(&captures[1]),
            flow,
            mask: 0,
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
        let curr_valve = &self.valves_with_flow.iter().find(|v| v.name == *cave).unwrap();
        let mut best_path = path.clone();
        self.valves_with_flow.iter().for_each(|valve| {
            if valve.flow == 0 {
                return ;
            }
            if path.open_valves & valve.mask != 0 {
                return ;
            }
            let distance = curr_valve.tunnels[valve.id as usize];
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

#[derive(Hash, Eq, PartialEq, Debug)]
struct TwoPathsKey(u8, u8, u8, u8, u32);

#[derive(Clone)]
struct TwoPaths {
    human_path: Vec<u8>,
    human_pos: usize,
    human_elapsed: u8,
    ele_path: Vec<u8>,
    ele_pos: usize,
    ele_elapsed: u8,
    open_valves: u32,
    elapsed: u8,
    total_flow: i32,
}

impl TwoPaths {
    fn new(path_size: usize) -> TwoPaths {
        TwoPaths {
            human_path: vec![0; path_size],
            human_pos: 0,
            human_elapsed: 0,
            ele_path: vec![0; path_size],
            ele_pos: 0,
            ele_elapsed: 0,
            open_valves: 0,
            elapsed: 0,
            total_flow: 0,
        }
    }

    fn cache_key(&self) -> TwoPathsKey {
        TwoPathsKey(
            self.human_path[self.human_pos],
            self.human_elapsed,
            self.ele_path[self.ele_pos],
            self.ele_elapsed,
            self.open_valves,
        )
    }

    fn merge(&self, other: &TwoPaths) -> TwoPaths {
        let mut human_path = self.human_path.clone();
        human_path[self.human_pos + 1..self.human_pos + other.human_pos + 1]
            .copy_from_slice(&other.human_path[..other.human_pos]);
        let mut ele_path = self.ele_path.clone();
        ele_path[self.ele_pos + 1..self.ele_pos + other.ele_pos + 1]
            .copy_from_slice(&other.ele_path[..other.ele_pos]);
        TwoPaths {
            human_path,
            human_pos: self.human_pos + other.human_pos,
            human_elapsed: self.human_elapsed + other.human_elapsed,
            ele_path,
            ele_pos: self.ele_pos + other.ele_pos,
            ele_elapsed: self.ele_elapsed + other.ele_elapsed,
            open_valves: self.open_valves,
            elapsed: self.elapsed + other.elapsed,
            total_flow: self.total_flow + other.total_flow,
        }
    }

    fn next_human(&self, valve: &Valve, distance: u8) -> TwoPaths {
        let mut human_path = self.human_path.clone();
        human_path[self.human_pos + 1] = valve.id;
        let ele_path = self.ele_path.clone();
        let elapsed = self.human_elapsed + distance + 1;
        let flow = (PART2_MINUTES - elapsed as i32) * valve.flow as i32;
        TwoPaths {
            human_path,
            human_pos: self.human_pos + 1,
            human_elapsed: elapsed,
            ele_path,
            ele_pos: self.ele_pos,
            ele_elapsed: self.ele_elapsed,
            open_valves: self.open_valves | valve.mask,
            elapsed: max(elapsed, self.ele_elapsed),
            total_flow: self.total_flow + flow
        }
    }

    fn next_elephant(&self, valve: &Valve, distance: u8) -> TwoPaths {
        let human_path = self.human_path.clone();
        let mut ele_path = self.ele_path.clone();
        ele_path[self.ele_pos + 1] = valve.id;
        let elapsed = self.ele_elapsed + distance + 1;
        let flow = (PART2_MINUTES - elapsed as i32) * valve.flow as i32;
        TwoPaths {
            human_path,
            human_pos: self.human_pos,
            human_elapsed: self.human_elapsed,
            ele_path,
            ele_pos: self.ele_pos + 1,
            ele_elapsed: elapsed,
            open_valves: self.open_valves | valve.mask,
            elapsed: max(elapsed, self.human_elapsed),
            total_flow: self.total_flow + flow
        }
    }

    fn diff(&self, start: &TwoPaths) -> TwoPaths {
        if self.human_pos < start.human_pos {
            panic!("Impossible condition: new human pos is less than start: {} = {}", 
                   self.human_pos, start.human_pos);
        }
        if self.ele_pos < start.ele_pos {
            panic!("Impossible condition: new elephant pos is less than start: {} = {}",
                   self.ele_pos, start.ele_pos);
        }
        let mut human_path = vec![0; self.human_path.len()];
        let hum_extra_walked = self.human_pos - start.human_pos;
        if hum_extra_walked > 0 {
            human_path[0..hum_extra_walked].copy_from_slice(&self.human_path[self.human_pos - 
                hum_extra_walked + 1..self.human_pos + 1]);
        }
        let mut ele_path = vec![0; self.human_path.len()];
        let ele_extra_walked = self.ele_pos - start.ele_pos;
        if ele_extra_walked > 0 {
            ele_path[0..ele_extra_walked].copy_from_slice(&self.ele_path[self.ele_pos - 
                ele_extra_walked + 1..self.ele_pos + 1]);
        }
        TwoPaths {
            human_path,
            human_pos: hum_extra_walked,
            human_elapsed: self.human_elapsed - start.human_elapsed,
            ele_path,
            ele_pos: ele_extra_walked,
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
        let cache_val = self.cache.get(&cache_key);
        if cache_val.is_some() {
            self.cache_hits += 1;
            return path.merge(cache_val.unwrap());
        }

        let man_pos = path.human_path[path.human_pos];
        let man_valve = &self.valves_with_flow[man_pos as usize];
        let ele_pos = path.ele_path[path.ele_pos];
        let ele_valve = &self.valves_with_flow[ele_pos as usize];

        let mut best_path = path.clone();
        self.valves_with_flow.iter().for_each(|valve| {
            if valve.flow == 0 {
                return ;
            }
            // try to move both human and elephant towards the next valve
            if (path.open_valves & valve.mask) != 0 {
                return ;
            }
            // move human
            let distance = man_valve.tunnels[valve.id as usize];
            let next = path.next_human(valve, distance);
            if next.elapsed < PART2_MINUTES as u8 {
                let sub_best = self.find_path(data, next);
                if sub_best.total_flow > best_path.total_flow {
                    best_path = sub_best;
                }
            }

            // move elephant
            let distance = ele_valve.tunnels[valve.id as usize];
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_valve(id: u8, name: &str, flow: u8, mask: u32) -> Valve {
        Valve {
            id,
            tunnels: Vec::new(),
            name: String::from(name),
            flow,
            mask,
        }
    }

    #[test]
    fn test_two_paths_new() {
        let path = TwoPaths::new(5);
        assert_eq!(path.human_path.len(), 5);
        assert_eq!(path.ele_path.len(), 5);
        assert_eq!(path.human_pos, 0);
        assert_eq!(path.ele_pos, 0);
        assert_eq!(path.open_valves, 0);
        assert_eq!(path.total_flow, 0);
    }

    #[test]
    fn test_two_paths_cache_key() {
        let mut path = TwoPaths::new(5);
        path.human_path[0] = 1;
        path.human_elapsed = 10;
        path.ele_path[0] = 2;
        path.ele_elapsed = 15;
        path.open_valves = 0x3;

        let key = path.cache_key();
        assert_eq!(key, TwoPathsKey(1, 10, 2, 15, 0x3));
    }

    #[test]
    fn test_two_paths_next_human() {
        let path = TwoPaths::new(5);
        let valve = create_test_valve(1, "BB", 10, 0x2);

        let next = path.next_human(&valve, 2);

        assert_eq!(next.human_path[1], 1);  // new position should be valve's ID
        assert_eq!(next.human_pos, 1);  // position incremented
        assert_eq!(next.human_elapsed, 3);  // distance + 1
        assert_eq!(next.ele_pos, 0);    // position unchanged
        assert_eq!(next.ele_elapsed, 0);    // position unchanged
        assert_eq!(next.open_valves, 0x2);  // valve's mask
        assert_eq!(next.total_flow, (PART2_MINUTES - 3) * 10);  // (26 - 3) * 10
    }

    #[test]
    fn test_two_paths_next_elephant() {
        let path = TwoPaths::new(5);
        let valve = create_test_valve(1, "BB", 10, 0x2);

        let next = path.next_elephant(&valve, 2);

        assert_eq!(next.ele_path[1], 1);
        assert_eq!(next.ele_pos, 1);
        assert_eq!(next.ele_elapsed, 3);
        assert_eq!(next.human_pos, 0);
        assert_eq!(next.human_elapsed, 0);
        assert_eq!(next.open_valves, 0x2);
        assert_eq!(next.total_flow, (PART2_MINUTES - 3) * 10);
    }

    #[test]
    fn test_two_paths_merge() {
        let mut path1 = TwoPaths::new(5);
        path1.human_path[0] = 1;
        path1.human_pos = 0;
        path1.human_elapsed = 5;
        path1.ele_path[0] = 2;
        path1.ele_pos = 0;
        path1.ele_elapsed = 6;
        path1.total_flow = 100;

        let mut path2 = TwoPaths::new(5);
        path2.human_path[0] = 3;
        path2.human_pos = 1;
        path2.human_elapsed = 3;
        path2.ele_path[0] = 4;
        path2.ele_pos = 1;
        path2.ele_elapsed = 4;
        path2.total_flow = 50;

        let merged = path1.merge(&path2);

        assert_eq!(merged.human_path[0], 1);
        assert_eq!(merged.human_path[1], 3);
        assert_eq!(merged.human_pos, 1);
        assert_eq!(merged.human_elapsed, 8);
        assert_eq!(merged.ele_path[0], 2);
        assert_eq!(merged.ele_path[1], 4);
        assert_eq!(merged.ele_pos, 1);
        assert_eq!(merged.ele_elapsed, 10);
        assert_eq!(merged.total_flow, 150);
    }

    #[test]
    fn test_two_paths_merge_no_move() {
        let mut path1 = TwoPaths::new(5);
        path1.human_path[0] = 1;
        path1.human_pos = 0;
        path1.human_elapsed = 5;
        path1.ele_path[0] = 2;
        path1.ele_pos = 0;
        path1.ele_elapsed = 6;
        path1.total_flow = 100;

        let mut path2 = TwoPaths::new(5);
        path2.human_pos = 0;
        path2.human_elapsed = 0;
        path2.ele_path[0] = 4;
        path2.ele_pos = 1;
        path2.ele_elapsed = 4;
        path2.total_flow = 50;

        let merged = path1.merge(&path2);

        assert_eq!(merged.human_path[0], 1);
        assert_eq!(merged.human_pos, 0);
        assert_eq!(merged.human_elapsed, 5);
        assert_eq!(merged.ele_path[0], 2);
        assert_eq!(merged.ele_path[1], 4);
        assert_eq!(merged.ele_pos, 1);
        assert_eq!(merged.ele_elapsed, 10);
        assert_eq!(merged.total_flow, 150);
    }

    #[test]
    fn test_two_paths_diff() {
        let mut path1 = TwoPaths::new(5);
        path1.human_path[0] = 1;
        path1.human_path[1] = 2;
        path1.human_pos = 1;
        path1.human_elapsed = 5;
        path1.ele_path[0] = 2;
        path1.ele_path[1] = 3;
        path1.ele_pos = 1;
        path1.ele_elapsed = 6;
        path1.total_flow = 100;

        let mut path2 = TwoPaths::new(5);
        path2.human_path[0] = 1;
        path2.human_pos = 0;
        path2.human_elapsed = 1;
        path2.ele_path[0] = 2;
        path2.ele_pos = 0;
        path2.ele_elapsed = 1;
        path2.total_flow = 30;

        let diff = path1.diff(&path2);

        assert_eq!(diff.human_path[0], 2);
        assert_eq!(diff.human_pos, 1);
        assert_eq!(diff.human_elapsed, 4);
        assert_eq!(diff.ele_path[0], 3);
        assert_eq!(diff.ele_pos, 1);
        assert_eq!(diff.ele_elapsed, 5);
        assert_eq!(diff.total_flow, 70);
    }

    #[test]
    fn test_two_paths_diff_human_hasnt_moved() {
        let mut path1 = TwoPaths::new(5);
        path1.human_path[0] = 1;
        path1.human_path[1] = 2;
        path1.human_pos = 1;
        path1.human_elapsed = 5;
        path1.ele_path[0] = 2;
        path1.ele_path[1] = 3;
        path1.ele_pos = 1;
        path1.ele_elapsed = 6;
        path1.total_flow = 100;

        let mut path2 = TwoPaths::new(5);
        path2.human_path[0] = 1;
        path2.human_path[1] = 2;
        path2.human_pos = 1;
        path2.human_elapsed = 5;
        path2.ele_path[0] = 2;
        path2.ele_pos = 0;
        path2.ele_elapsed = 2;
        path2.total_flow = 30;

        let diff = path1.diff(&path2);

        assert_eq!(diff.human_pos, 0);
        assert_eq!(diff.human_elapsed, 0);
        assert_eq!(diff.ele_path[0], 3);
        assert_eq!(diff.ele_pos, 1);
        assert_eq!(diff.ele_elapsed, 4);
        assert_eq!(diff.total_flow, 70);
    }
}
