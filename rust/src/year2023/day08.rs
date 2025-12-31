// https://adventofcode.com/2023/day/8

use std::collections::HashMap;

use log::{debug, info};

use adventofcode::Solver;

pub struct Solution {
    steps: Vec<char>,
    nodes: HashMap<String, Node>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            steps: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    fn next_node(&self, pos: &str, step: usize) -> &str {
        let node = self.nodes.get(pos).unwrap();
        let dir = self.steps[step % self.steps.len()];
        match dir {
            'L' => node.left.as_str(),
            'R' => node.right.as_str(),
            _ => panic!("Invalid direction? {}", dir),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        if self.steps.len() == 0 {
            self.steps = line.chars().collect();
            return;
        }

        // parse AAA = (BBB, BBB)
        let mut parts = line.split(" ");
        let name = parts.next().unwrap();
        assert_eq!(parts.next().unwrap(), "=");
        let left = parts.next().unwrap()
            .trim_start_matches("(")
            .trim_end_matches(",");
        let right = parts.next().unwrap()
            .trim_end_matches(")");
        self.nodes.insert(String::from(name), Node {
            left: String::from(left),
            right: String::from(right)
        });
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("Steps: {} - nodes: {}", self.steps.len(), self.nodes.len());

        let mut steps1 = 0;
        let mut pos = "AAA";
        while pos != "ZZZ" {
            pos = self.next_node(pos, steps1);
            steps1 += 1;
        }
        info!("[1] Reached the end in {} steps", steps1);

        // let's try part 2 without loop checks...
        let mut ghosts: Vec<&str> = self.nodes.keys()
            .filter(|k| k.ends_with("A"))
            .map(|k| k.as_str())
            .collect();
        debug!("Part 2 - intial ghosts: {}", ghosts.len());
        let mut steps2 = 0;
        let mut cycles = 1_usize;
        while !all_finished(&ghosts) {
            let new_ghosts: Vec<&str> = ghosts.iter()
                .map(|pos| self.next_node(pos, steps2))
                .collect();
            steps2 += 1;
            let (cycling, wandering): (Vec<_>, Vec<_>) = new_ghosts.into_iter()
                .partition(|pos| pos.ends_with("Z"));
            if cycling.len() > 0 {
                cycles = num::integer::lcm(cycles, steps2);
                debug!("Found {} ghosts cycling at step {steps2} - {} still wandering (=> lcm {cycles})",
                    cycling.len(), wandering.len());
            }
            ghosts = wandering;
        }
        info!("[2] Reached the end in {} steps", steps2);

        Some((steps1.to_string(), steps2.to_string()))
    }
}

fn all_finished(ghosts: &Vec<&str>) -> bool {
    ghosts.iter().all(|pos| pos.ends_with("Z"))
}

struct Node {
    left: String,
    right: String,
}