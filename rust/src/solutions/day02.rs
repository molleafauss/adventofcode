// What did I learn?
// matching on multiple variables, a bit of borrowing for function parameters

use crate::Solver;

pub struct Solution {
    score1: u32,
    score2: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {score1: 0, score2: 0}
    }
}

#[derive(Debug)]
enum Symbol {
    Rock,
    Paper,
    Scissors
}

fn score_played(symbol: &Symbol) -> u32 {
    match symbol {
        Symbol::Rock => 1,
        Symbol::Paper => 2,
        Symbol::Scissors => 3,
    }
}

fn score_round2(result: &str) -> u32 {
    match result {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("Invalid round2 score: {result}")
    }
}

fn map_opponent(input: &str) -> Symbol {
    match input {
        "A" => Symbol::Rock,
        "B" => Symbol::Paper,
        "C" => Symbol::Scissors,
        _ => panic!("Invalid opponent input: {input}")
    }
}

fn map_play(input: &str) -> Symbol {
    match input {
        "X" => Symbol::Rock,
        "Y" => Symbol::Paper,
        "Z" => Symbol::Scissors,
        _ => panic!("Invalid play input: {input}")
    }
}

fn score_result(opponent: &Symbol, mine: &Symbol) -> u32 {
    match (opponent, mine) {
        (Symbol::Rock, Symbol::Rock) => 3,
        (Symbol::Rock, Symbol::Paper) => 6,
        (Symbol::Rock, Symbol::Scissors) => 0,
        (Symbol::Paper, Symbol::Rock) => 0,
        (Symbol::Paper, Symbol::Paper) => 3,
        (Symbol::Paper, Symbol::Scissors) => 6,
        (Symbol::Scissors, Symbol::Rock) => 6,
        (Symbol::Scissors, Symbol::Paper) => 0,
        (Symbol::Scissors, Symbol::Scissors) => 3,
    }

}

fn play_round2(opponent: &Symbol, score: &u32) -> Symbol {
    match (opponent, score) {
        (Symbol::Rock, 3) => Symbol::Rock,
        (Symbol::Rock, 6) => Symbol::Paper,
        (Symbol::Rock, 0) => Symbol::Scissors,
        (Symbol::Paper, 0) => Symbol::Rock,
        (Symbol::Paper, 3) => Symbol::Paper,
        (Symbol::Paper, 6) => Symbol::Scissors,
        (Symbol::Scissors, 6) => Symbol::Rock,
        (Symbol::Scissors, 0) => Symbol::Paper,
        (Symbol::Scissors, 3) => Symbol::Scissors,
        _ => panic!("Invalid round2 play: {:?} {score}", opponent)
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let opponent_play = map_opponent(&parts[0]);
        let my_play = map_play(&parts[1]);
        let score_round2 = score_round2(&parts[1]);
        let round2_play = play_round2(&opponent_play, &score_round2);
        self.score1 += score_played(&my_play) + score_result(&opponent_play, &my_play);
        self.score2 += score_played(&round2_play) + score_round2;
    }

    fn solve(&mut self) {
        println!("[1] Resulting score (part 1): {}", self.score1);
        println!("[2] Resulting score (part 2): {}", self.score2)
    }
}
