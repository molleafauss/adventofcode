// What did I learn?
// A bit of Result usage to handle "errors" (= using error to break from recursive calls)

use std::collections::HashMap;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use crate::Solver;

pub(crate) struct Solution {
    monkeys: HashMap<String, Monkey>
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            monkeys: HashMap::new(),
        }
    }

    fn calculate(&self, monkey: &Monkey, part2: bool) -> Result<f64, &str>{
        if part2 && monkey.name == HUMAN {
            return Err("Hooman do not know number");
        }

        let result = match &monkey.action {
            Action::Number(num) => *num,
            Action::Calculus(left, op, right) => {
                let left_res = self.calculate(&self.monkeys[left.as_str()], part2)?;
                let right_res = self.calculate(&self.monkeys[right.as_str()], part2)?;
                match op {
                    Op::Sum => left_res + right_res,
                    Op::Subtract => left_res - right_res,
                    Op::Multiply => left_res * right_res,
                    Op::Divide => left_res / right_res,
                }
            }
        };
        Ok(result)
    }

    fn balance(&self, m: &Monkey) -> f64 {
        let mut monkey = m;
        // which branch is the one missing data?
        let mut balance = None;
        let mut value;
        while monkey.name != HUMAN {
            let Action::Calculus(m_left, op, m_right) = &monkey.action else {
                panic!("Monkey Action is not a calculation: {} => {:?}", monkey.name, monkey.action);
            };
            let mut human = m_right;
            match self.calculate(&self.monkeys[m_left], true) {
                Ok(val) => { value = Some(val); },
                Err(_) => {
                    human = m_left;
                    // this should not fail
                    value = Some(self.calculate(&self.monkeys[m_right], true).unwrap());
                }
            }
            if balance.is_none() {
                println!("Found root branch value: {}", value.unwrap());
                balance = value;
            } else {
                balance = self.invert_op(balance.unwrap(), op, value.unwrap(), human == m_left);
            }
            monkey = &self.monkeys[human];
        }
        balance.unwrap()
    }

    fn invert_op(&self, balance: f64, op: &Op, value: f64, first: bool) -> Option<f64> {
        let v_str = value.to_string();
        println!("Inverting {} {:?} {} = {balance}",
                 if first { "x" } else { &v_str }, op, if first { &v_str } else { "x" });
        match op {
            Op::Sum => Some(balance - value),
            Op::Subtract => Some(if first { balance + value } else { value - balance }),
            Op::Multiply => Some(balance / value),
            Op::Divide => Some(if first { balance * value } else { value / balance }),
        }
    }
}

static HUMAN: &str = "humn";
static ROOT: &str = "root";
static RE_OP: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\S+) ([+\-*/]) (\S+)").unwrap());

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let (name, op) = line.split_once(": ").unwrap();
        if let Some(captures) = RE_OP.captures(op) {
            let m = Monkey::with_op(name, captures);
            self.monkeys.insert(m.name.clone(), m);
        } else {
            let m = Monkey::with_num(name, f64::from_str(op).unwrap());
            self.monkeys.insert(m.name.clone(), m);
        }

    }

    fn solve(&mut self) {
        assert!(self.monkeys.contains_key(ROOT), "Missing root in monkeys?");

        // part 1 - must not fail
        let result = self.calculate(&self.monkeys[ROOT], false).unwrap();
        println!("[1] Result is {result}");

        // part 2
        let value = self.balance(&self.monkeys[ROOT]);
        println!("[2] HUMN {value}");

    }
}

#[derive(Debug)]
enum Op {
    Sum,
    Subtract,
    Multiply,
    Divide,
}

impl Op {
    fn parse(text: &str) -> Op {
        match text {
            "+" => Op::Sum,
            "-" => Op::Subtract,
            "*" => Op::Multiply,
            "/" => Op::Divide,
            _ => panic!("Invalid operation: {text}"),
        }
    }
}

#[derive(Debug)]
enum Action {
    Number(f64),
    Calculus(String, Op, String),
}

struct Monkey {
    name: String,
    action: Action,
}

impl Monkey {
    fn with_op(name: &str, op: Captures) -> Monkey {
        let action = Action::Calculus(
            String::from(&op[1]),
            Op::parse(&op[2]),
            String::from(&op[3]),
        );
        Monkey {
            name: String::from(name),
            action,
        }
    }

    fn with_num(name: &str, num: f64) -> Monkey {
        Monkey {
            name: String::from(name),
            action: Action::Number(num),
        }
    }
}
