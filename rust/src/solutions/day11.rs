// What did I learn?
// Had to implement the operation but was _reasonably_ simple (and indeed frigging faster than python eval)
// math needed to be done in u64 to avoid overflowing
// still not clear if there's a idiomatic/library function tp print values from a Vec...

use std::str::FromStr;
use std::usize;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::Solver;

pub(crate) struct Solution {
    monkeys: Vec<Monkey>
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            monkeys: Vec::new()
        }
    }

    fn run_loops(&mut self, iterations: u32, reduce_worry: u64, reducer: u64) -> u64 {
        self.monkeys.iter_mut().for_each(|m| m.start());
        (0..iterations).for_each(|i| {
            // not clear if I can use an iter here
            (0..self.monkeys.len()).for_each(|i| {
                self.monkeys[i].act(reduce_worry, reducer)
                    .iter()
                    .for_each(|(id, item)| self.monkeys[*id].add_item(*item));
            });

            if i % 1000 == 0 {
                let mut vals = String::from("[");
                self.monkeys.iter().for_each(|m| {
                    vals.push_str(&m.inspected.to_string());
                    vals.push_str(", ")
                });
                vals.push_str("]");
                println!("[{}] {}", i, vals);
            }
        });

        let mut result: Vec<(usize, u32)> = self.monkeys.iter()
            .map(|m| (m.id, m.inspected))
            .collect();
        result.sort_by_key(|it| it.1);
        result.reverse();
        println!("1st {} => {}", result[0].0, result[0].1);
        println!("2nd {} => {}", result[1].0, result[1].1);
        result[0].1 as u64 * result[1].1 as u64
    }
}

static RE_MONKEY: Lazy<Regex> = Lazy::new(|| Regex::new(r"Monkey (\d+):").unwrap());
static RE_ITEMS: Lazy<Regex> = Lazy::new(|| Regex::new("Starting items: (.*)").unwrap());
static RE_OPERATION: Lazy<Regex> = Lazy::new(|| Regex::new("Operation: new = (.*)").unwrap());
static RE_TEST: Lazy<Regex> = Lazy::new(|| Regex::new(r"Test: divisible by (\d*)").unwrap());
static RE_RESULT: Lazy<Regex> = Lazy::new(|| Regex::new(r"If (false|true): throw to monkey (\d*)").unwrap());

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if let Some(matching) = RE_MONKEY.captures(line) {
            self.monkeys.push(Monkey::new(&matching[1]));
        } else if let Some(matching) = RE_ITEMS.captures(line) {
            self.monkeys.last_mut().unwrap().set_items(&matching[1]);
        } else if let Some(matching) = RE_OPERATION.captures(line) {
            self.monkeys.last_mut().unwrap().set_operation(&matching[1]);
        } else if let Some(matching) = RE_TEST.captures(line) {
            self.monkeys.last_mut().unwrap().set_test(&matching[1]);
        } else if let Some(matching) = RE_RESULT.captures(line) {
            self.monkeys.last_mut().unwrap().set_result(&matching[1], &matching[2]);
        } else if !line.is_empty() {
            panic!("Unparseable line: {line}");
        }
    }

    fn solve(&mut self) {
        let reducer = self.monkeys.iter().map(|m| m.test.0).product();
        println!("Reducer: {reducer}");

        let total_inspected = self.run_loops(20, 3, reducer);
        println!("[1] top 2 inspected: {total_inspected}");

        let total_inspected = self.run_loops(10000, 1, reducer);
        println!("[2] top 2 inspected: {total_inspected}");
    }
}

struct Monkey {
    id: usize,
    initial_items: Vec<u64>,
    operation: Operation,
    // divisor, send when true, send when false
    test: (u64, usize, usize),
    // working items
    items: Vec<u64>,
    // number of inspected items
    inspected: u32,
}

impl Monkey {
    fn new(id: &str) -> Monkey {
        Monkey {
            id: usize::from_str(id).unwrap(),
            initial_items: Vec::new(),
            operation: Operation::new(),
            test: (0, 0, 0),
            items: Vec::new(),
            inspected: 0,
        }
    }

    fn set_items(&mut self, items: &str) {
        items.split(", ").for_each(|it| self.initial_items.push(u64::from_str(it).unwrap()));
    }

    fn set_operation(&mut self, operation: &str) {
        println!("Operation for monkey {}: {operation}", self.id);
        self.operation.parse(operation);
    }

    fn set_test(&mut self, divisor: &str) {
        self.test.0 = u64::from_str(divisor).unwrap();
    }

    fn set_result(&mut self, result: &str, destination: &str) {
        match result {
            "true"  => self.test.1 = usize::from_str(destination).unwrap(),
            "false" => self.test.2 = usize::from_str(destination).unwrap(),
            _ => panic!("Invalid result: {result}"),
        }
    }

    fn start(&mut self) {
        self.items = self.initial_items.to_vec();
        self.inspected = 0;
    }

    fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }

    fn act(&mut self, reduce_worry: u64, reducer: u64) -> Vec<(usize, u64)> {
        let mut throws = Vec::new();
        self.items.drain(..).for_each(|old| {
            self.inspected += 1;
            let new = self.operation.calculate(old) % reducer;
            let pass = new / reduce_worry;
            // println!("[Monkey {}] {} => {} => {}: test {} => pass to {}", self.id, old, new, pass, pass % self.test.0,
            //          if (pass % self.test.0) == 0 { self.test.1 } else { self.test.2 });
            if (pass % self.test.0) == 0 {
                throws.push((self.test.1, pass));
            } else {
                throws.push((self.test.2, pass));
            }
        });
        throws
    }
}

enum OpType {
    Add,
    Multiply,
}

enum Operand {
    Input,
    Value(u64)
}

struct Operation {
    op: OpType,
    rhs: Operand,
}

impl Operation {
    fn new() -> Operation {
        Operation {
            op: OpType::Add,
            rhs: Operand::Value(0),
        }
    }

    // supports old (+*) (number|old)
    fn parse(&mut self, expression: &str) {
        let mut parts = expression.split_ascii_whitespace();
        if parts.next().unwrap() != "old" {
            panic!("Unsupported operation? (old not lhs?) {expression}");
        }
        match parts.next().unwrap() {
            "+" => self.op = OpType::Add,
            "*" => self.op = OpType::Multiply,
            _ => panic!("Unsupported operation? (unknown optype?) {expression}")
        }
        let lhs = parts.next().unwrap();
        if lhs == "old" {
            self.rhs = Operand::Input;
        } else {
            self.rhs = Operand::Value(u64::from_str(lhs).unwrap());
        }
    }

    fn calculate(&self, old: u64) -> u64 {
        let lhs = match self.rhs {
            Operand::Input => old,
            Operand::Value(x) => x,
        };
        match self.op {
            OpType::Add => old + lhs,
            OpType::Multiply => old * lhs,
        }
    }
}