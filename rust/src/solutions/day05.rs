// What did I learn?
// using the regex create, some fun with manipulating chars (should have I used u8 directly?)
// still have to get decent sensibility on wether is better to use chars or directly u8 from a &str...
// learned get_mut() on Vec to mutate internal elements.
// attention to mutability borrow when you have both source and destination to change in the multiple moves
// (need to pick what will change first and then apply it)

use std::cmp::max;
use std::str::FromStr;
use std::usize;
use once_cell::sync::Lazy;
use crate::Solver;
use regex::Regex;

static RE_INSTRUCTION: Lazy<Regex> = Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

pub struct Solution {
    parse_status: ParseStatus,
    stack_defs: Vec<String>,
    instructions: Vec<Instruction>,
}

enum ParseStatus {
    StackDefs,
    Instructions,
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            parse_status: ParseStatus::StackDefs,
            stack_defs: Vec::new(),
            instructions: Vec::new(),
        }
    }

    fn build_stacks(&mut self) -> Vec<Vec<char>> {
        let num_stacks = (self.stack_defs[0].len() - 3) / 4 + 1;
        println!("Creating {num_stacks} stacks");
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

        // ignore last line
        let mut i = 1;
        while i < self.stack_defs.len() {
            i += 1;
            let row = self.stack_defs.len() - &i;
            let line = self.stack_defs.get(row).unwrap();
            let mut chars = line.chars();
            // consume first character
            for i in 0..num_stacks {
                // get 4th character
                let idx = if i == 0 { 1 } else { 3 };
                let val = chars.nth( idx).unwrap().into();
                if val != ' ' {
                    stacks.get_mut(i).unwrap().push(val);
                }
            }
        }

        stacks
    }

    fn move_singles(&mut self, mut stacks: Vec<Vec<char>>) -> String {
        for action in self.instructions.iter() {
            let mut moves = action.amount.clone();
            while moves > 0 {
                let source = stacks.get_mut(action.from - 1).unwrap().pop();
                if source.is_some() {
                    stacks.get_mut(action.to - 1).unwrap().push(source.unwrap());
                }
                moves -= 1;
            }
        }
        let mut result = String::new();
        for stack in stacks.iter_mut() {
            result.push(stack.pop().unwrap());
        }
        result
    }

    fn move_multiples(&mut self, mut stacks: Vec<Vec<char>>) -> String {
        for action in self.instructions.iter() {
            let source = stacks.get_mut(action.from - 1).unwrap();
            let mut moved: Vec<char> = Vec::new();
            // find how many items to remove from source. Don't remove any if you don't have enough
            let pos = source.len() as i32 - action.amount as i32;
            let mut items = action.amount as i32;
            while items > 0 {
                let remove_idx = max(pos, 0) as usize;
                moved.push(source.remove(remove_idx));
                items -= 1;
            }
            let dest = stacks.get_mut(action.to - 1).unwrap();
            moved.drain(0..).for_each(|item| dest.push(item.clone()));
        }

        let mut result = String::new();
        for stack in stacks.iter_mut() {
            result.push(stack.pop().unwrap());
        }
        result
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            self.parse_status = ParseStatus::Instructions;
            return;
        }
        match self.parse_status {
            ParseStatus::StackDefs => self.stack_defs.push(String::from(line)),
            ParseStatus::Instructions => self.instructions.push(parse_instruction(line)),
        }
    }

    fn solve(&mut self) {
        let mut stacks = self.build_stacks();
        let part1 = self.move_singles(stacks);
        println!("[1] Top stacks values: {part1}");
        stacks = self.build_stacks();
        let part2 = self.move_multiples(stacks);
        println!("[2] Top stacks values: {part2}");
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let matching = RE_INSTRUCTION.captures(line);
    if matching.is_none() {
        panic!("Line {line} does not match!");
    }
    let captures = matching.unwrap();
    Instruction {
        amount: usize::from_str(&captures[1]).unwrap(),
        from: usize::from_str(&captures[2]).unwrap(),
        to: usize::from_str(&captures[3]).unwrap(),
    }
}