// What did I learn?
// integer math with rounding - needed to find a formula to crack it (// is not supported in rust
// and div_ceil is experimental.
// used slices for some variables - finally got some understanding of those.
// Interesting enough, even in debug times are quite fast (69 vs 576 sec for test input, 5 vs 42 sec
// for challenge input => release drops times to 12 & 0.7).

use std::collections::HashMap;
use std::str::FromStr;
use std::time::SystemTime;
use log::{debug, info};
use once_cell::sync::Lazy;
use regex::Regex;
use crate::Solver;

pub(crate) struct Solution {
    blueprints: Vec<Blueprint>
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            blueprints: Vec::new(),
        }
    }

    fn find_max_geodes(&self, stats: &mut Stats, bp: &Blueprint, minutes_left: i32, robots: [i32; 4], materials: [i32; 4]) -> i32 {
        stats.calls += 1;
        if stats.calls % 500000 == 0 {
            println!("{} calls, {} cache hits", stats.calls, stats.cache_hits);
        }
        assert!(minutes_left > 0);
        // if we're at time, just return what we have
        if minutes_left == 0 {
            return  materials[GEODE];
        }
        // have we seen this status?
        let cache_key = CacheKey (
            minutes_left,
            robots.clone(),
            materials[ORE].min(bp.max_materials[ORE] * minutes_left),
            materials[CLAY].min(bp.max_materials[CLAY] * minutes_left),
            materials[OBSIDIAN].min(bp.max_materials[OBSIDIAN] * minutes_left),
            materials[GEODE]
        );
        if stats.cache.contains_key(&cache_key) {
            stats.cache_hits += 1;
            return stats.cache[&cache_key];
        }
        // start with maximum that can be produced by the current status
        let mut max_geodes = materials[GEODE] + (robots[GEODE] * minutes_left);
        for bot_type in MATERIALS {
            let recipe = &bp.recipes[bot_type];
            if bot_type != GEODE && robots[bot_type] >= bp.max_materials[bot_type] {
                // culling - building robots of a type over the maximum consumption of a material is not necessary
                continue;
            }
            let mut time_needed = 1;
            for mat in MATERIALS {
                if robots[mat] == 0 && recipe[mat] > 0 {
                    // can't produce
                    time_needed = -1;
                    break;
                }
                if recipe[mat] == 0 {
                    continue;
                }
                // count how many more materials I need and then divide by existing robot - always rounding up
                // int divide a/b rounding up -> (a + b - 1) / b
                let rounds = (recipe[mat] - materials[mat] + robots[mat] - 1) / robots[mat] + 1;
                time_needed = time_needed.max(rounds);
            }
            if time_needed > 0 && minutes_left - time_needed > 0 {
                let mut new_robots = robots.clone();
                new_robots[bot_type] += 1;
                let new_materials = [
                    materials[ORE] + (robots[ORE] * time_needed) - recipe[ORE],
                    materials[CLAY] + (robots[CLAY] * time_needed) - recipe[CLAY],
                    materials[OBSIDIAN] + (robots[OBSIDIAN] * time_needed) - recipe[OBSIDIAN],
                    materials[GEODE] + (robots[GEODE] * time_needed) - recipe[GEODE],
                ];
                max_geodes = max_geodes.max(
                    self.find_max_geodes(stats, bp, minutes_left - time_needed, new_robots, new_materials)
                );
            }
        }

        stats.cache.insert(cache_key, max_geodes);
        max_geodes
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        self.blueprints.push(Blueprint::parse(line));
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let mut total1 = 0;
        let mut total2 = 1;
        let mut part2 = 0;
        for bp in &self.blueprints {
            debug!("Finding max geodes for blueprint {} => {:?}", bp.id, bp.recipes);
            let robots = [1, 0, 0, 0];
            let material = [0, 0, 0, 0];
            let mut stats = Stats { calls: 0, cache_hits: 0, cache: HashMap::new() };
            let t0 = SystemTime::now();
            let max_geodes = self.find_max_geodes(&mut stats, bp, 24, robots, material);
            total1 += max_geodes * bp.id;
            let t1 = SystemTime::now();
            let diff_sec = t1.duration_since(t0).unwrap().as_secs_f32();
            let iter_time = 1000000.0 * diff_sec / stats.calls as f32;
            debug!("[part 1] Blueprint {} => {max_geodes} ({total1}) [{:.3}sec {} total calls / {:.3} us/call / {} cache hits]",
                     bp.id, diff_sec, stats.calls, iter_time, stats.cache_hits);

            if part2 < 3 {
                part2 += 1;
                let mut stats = Stats { calls: 0, cache_hits: 0, cache: HashMap::new() };
                let t0 = SystemTime::now();
                let max_geodes = self.find_max_geodes(&mut stats, bp, 32, robots, material);
                total2 *= max_geodes;
                let t1 = SystemTime::now();
                let diff_sec = t1.duration_since(t0).unwrap().as_secs_f32();
                let iter_time = 1000000.0 * diff_sec / stats.calls as f32;
                debug!("[part 2] Blueprint {} => {max_geodes} ({total2}) [{:.3}sec {} total calls / {:.3} us/call / {} cache hits]",
                         bp.id, diff_sec, stats.calls, iter_time, stats.cache_hits);
            }
        }
        info!("[1] result is {total1}");
        info!("[2] result is {total2}");
        Some((total1.to_string(), total2.to_string()))
    }
}

static RE_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"Blueprint (\d+): ").unwrap());
static RE_RECIPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Each ([a-z]+) robot costs ").unwrap());
static RE_COMP: Lazy<Regex> = Lazy::new(|| Regex::new(r"( and )?(\d+) ([a-z]+)").unwrap());
const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
const MATERIALS: [usize; 4] = [ORE, CLAY, OBSIDIAN, GEODE];

fn parse_type(text: &str) -> usize {
    match text {
        "ore" => ORE,
        "clay" => CLAY,
        "obsidian" => OBSIDIAN,
        "geode" => GEODE,
        _ => panic!("Unsupported material {}", text)
    }
}


struct Blueprint {
    id: i32,
    max_materials: [i32; 4],
    recipes: [[i32; 4]; 4],
}

impl Blueprint {
    fn parse(line: &str) -> Blueprint {
        let Some(id_capture) = RE_ID.captures(line) else {
            panic!("Invalid blueprint definition: {}", line);
        };
        let mut bp = Blueprint {
            id: i32::from_str(&id_capture[1]).unwrap(),
            max_materials: [0; 4],
            recipes: [[0; 4]; 4],
        };
        let mut idx = id_capture[0].len();
        while idx < line.len() {
            let Some(recipe) = RE_RECIPE.captures_at(line, idx) else {
                panic!("Cant parse recipe from {}: {}", idx, &line[idx..]);
            };
            let robot = parse_type(&recipe[1]);
            idx += recipe[0].len();
            while ! &line[idx..].starts_with(".")  {
                let Some(comp) = RE_COMP.captures_at(line, idx) else {
                    panic!("Can't parse component at {}: {}", idx, &line[idx..]);
                };
                let amount = i32::from_str(&comp[2]).unwrap();
                let mat = parse_type(&comp[3]);
                bp.recipes[robot][mat] = amount;
                bp.max_materials[mat] = bp.max_materials[mat].max(amount);
                idx += comp[0].len();
            }
            idx += 2;
        }
        bp
    }
}

#[derive(Eq, Hash, PartialEq)]
struct CacheKey(i32, [i32; 4], i32, i32, i32, i32);

struct Stats {
    calls: u32,
    cache: HashMap<CacheKey, i32>,
    cache_hits: u32,
}