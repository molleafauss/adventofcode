mod solutions;
mod grid;

use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;

use solutions::Solver;
use solutions::solver_for;

fn solve(filename: &str, mut parser: Box<dyn Solver>) {
    let mut expected_part_1 = None;
    let mut expected_part_2 = None;
    for line in fs::read_to_string(filename).unwrap().lines() {
        if line.starts_with("result part 1: ") {
            expected_part_1 = Some(String::from(&line[15..]));
        } else if line.starts_with("result part 2: ") {
            expected_part_2 = Some(String::from(&line[15..]));
        } else {
            parser.parse(line);
        }
    }
    let t0 = SystemTime::now();
    let result = parser.solve();
    let t1 = SystemTime::now();
    println!("File {filename}: {:.3}sec", t1.duration_since(t0).unwrap().as_secs_f32());
    if result.is_none() {
        return;
    }
    let (part1, part2) = result.unwrap();
    if let Some(expected1) = expected_part_1 {
        if part1 == expected1 {
            println!("PART 1 - found expected result: {expected1} = {part1}")
        } else {
            println!("ERROR - part 1 result is incorrect: expected {expected1}, actual {part1}");
        }
    }
    if let Some(expected2) = expected_part_2 {
        if part2 == expected2 {
            println!("PART 2 - found expected result: {expected2} = {part2}", )
        } else  {
            println!("ERROR - part 2 result is incorrect: expected {expected2}, actual {part2}");
        }
    }
}

fn solve_day(day: String) {
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

fn solve_all() {
    for day in 1..26 {
        solve_day(format!("day{:02}", day));
    }
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        println!("Please specify a day to resolve like 'day03'");
        return;
    }
    let day = args.nth(1).unwrap();
    if day == "all" {
        solve_all();
    } else {
        solve_day(day);
    }
}
