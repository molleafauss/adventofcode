use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;

use log::{error, info, LevelFilter, warn};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

pub mod grid;
pub mod utils;

/// A solver for a AOC puzzle
pub trait Solver {
    /// parse the given line
    fn parse(&mut self, line: &str);
    /// solve the puzzle
    fn solve(&mut self) -> Option<(String, String)>;
}

fn solve(filename: &str, mut parser: Box<dyn Solver>) {
    let mut expected_part_1 = None;
    let mut expected_part_2 = None;
    let t0 = SystemTime::now();
    for line in fs::read_to_string(filename).unwrap().lines() {
        if line.starts_with("result part 1: ") {
            expected_part_1 = Some(String::from(&line[15..]));
        } else if line.starts_with("result part 2: ") {
            expected_part_2 = Some(String::from(&line[15..]));
        } else {
            parser.parse(line);
        }
    }
    let result = parser.solve();
    let t1 = SystemTime::now();
    info!("File {filename}: {:.3}sec", t1.duration_since(t0).unwrap().as_secs_f32());
    if result.is_none() {
        warn!("==> No result given");
        return;
    }
    let (part1, part2) = result.unwrap();
    if let Some(expected1) = expected_part_1 {
        if part1 == expected1 {
            info!("PART 1 - found expected result: {expected1} = {part1}")
        } else {
            error!("ERROR - part 1 result is incorrect: expected {expected1}, actual {part1}");
        }
    }
    if let Some(expected2) = expected_part_2 {
        if part2 == expected2 {
            info!("PART 2 - found expected result: {expected2} = {part2}", )
        } else  {
            error!("ERROR - part 2 result is incorrect: expected {expected2}, actual {part2}");
        }
    }
}

fn solve_day(year: &str, day: String, solver_for: fn(day: &str) -> Box<dyn Solver>) {
    info!("== Solving {day} ==");

    // assume 'input' is a directory in the current directory
    let test_file = format!("inputs/{year}/{day}/test.txt");
    if !Path::new(&test_file).exists() {
        error!("ERROR: test file {test_file} does not exist");
        exit(-1);
    }
    solve(&test_file, solver_for(&day));

    let input_file = format!("inputs/{year}/{day}/input.txt");
    if !Path::new(&input_file).exists() {
        error!("ERROR: input file {input_file} does not exist");
        exit(-1);
    }
    solve(&input_file, solver_for(&day));
}

fn solve_all(year: &str, solver_for: fn(day: &str) -> Box<dyn Solver>) {
    for day in 1..26 {
        solve_day(year, format!("day{:02}", day), solver_for);
    }
}

fn init_logging(year: &str, level: LevelFilter) {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} | {t} | {m}{n}")))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build(format!("aoc{}", year), level))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(config).unwrap();
}

pub fn run(year: &str, solver_for: fn(day: &str) -> Box<dyn Solver>, level: LevelFilter) {
    let mut args = env::args();
    if args.len() < 2 {
        println!("Please specify a day to resolve like 'day03'");
        return;
    }
    init_logging(year, level);
    let day = args.nth(1).unwrap();
    if day == "all" {
        solve_all(year, solver_for);
    } else {
        solve_day(year, day, solver_for);
    }
}
