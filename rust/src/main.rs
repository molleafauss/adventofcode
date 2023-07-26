mod solutions;

use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

use solutions::Solver;
use solutions::solver_for;

fn solve(filename: &str, mut parser: impl Solver) {
    for line in fs::read_to_string(filename).unwrap().lines() {
        parser.parse(line);
    }
    parser.solve()
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        println!("Please specify a day to resolve like 'day03'");
        return;
    }
    let day = args.nth(1).unwrap();
    println!("== Solving {day} ==");

    // assume 'input' is a directory in the current directory
    let test_file = format!("inputs/{day}/test.txt");
    if !Path::new(&test_file).exists() {
        println!("ERROR: test file {test_file} does not exist");
        exit(-1);
    }
    solve(&test_file, solver_for(&day));

    let input_file = format!("inputs/{day}/input.txt");
    if !Path::new(&input_file).exists() {
        println!("ERROR: input file {input_file} does not exist");
        exit(-1);
    }
    solve(&input_file, solver_for(&day));
}
