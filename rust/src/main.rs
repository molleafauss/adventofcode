use adventofcode::{get_solver, Solver};
use chrono::{Datelike, Local};
use clap::Parser;
use log::{error, info, warn, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use std::fs;
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;
mod year2021;
mod year2022;
mod year2023;

fn register_all_solvers() {
    year2021::register_solvers();
    year2022::register_solvers();
    year2023::register_solvers();
}

/// Solves Advent of Code puzzles
#[derive(Parser)]
#[command(name = "adventofcode")]
struct Aoc {
    /// Directory to read input files from, default current directory.
    #[arg(long, default_value = "inputs")]
    inputs: String,

    /// Year of the Advent of Code event - default last available year.
    #[arg(long)]
    year: Option<u32>,

    /// Enable debug log.
    #[arg(long, default_value = "false")]
    debug: bool,

    /// Day to solve (specified as 'dayNN' or 'all' to solve all days in sequence).
    day: String,
}

fn main() {
    let cmd = Aoc::parse();
    cmd.run();
}

fn get_latest_year() -> u32 {
    let now = Local::now();
    if now.month() >= 12 {
        now.year() as u32
    } else {
        (now.year() - 1) as u32
    }
}

impl Aoc {
    pub(crate) fn run(&self) {
        init_logging(self.debug);

        register_all_solvers();

        // verify input data

        let year = self.year.unwrap_or_else(get_latest_year);
        if self.day == "all" {
            self.solve_all(year);
        } else if self.day.starts_with("day") {
            self.solve_day(year, &self.day);
        } else {
            error!(
                "Invalid day parameter: {}. Must be 'dayNN' or 'all'.",
                self.day
            );
            exit(-1);
        }
    }

    fn solve_all(&self, year: u32) {
        for day in 1..26 {
            self.solve_day(year, &format!("day{:02}", day));
        }
    }

    fn solve_day(&self, year: u32, day: &str) {
        let data = &day[..5];
        info!("== Solving {year} / {data} ==");
        let solver = get_solver(&year.to_string(), day);
        if solver.is_none() {
            warn!("{year}/{day} | no solution implemented");
            return;
        }
        let solver = solver.unwrap();
        self.solve(year, data, "test.txt", solver());
        self.solve(year, data, "input.txt", solver());
    }

    fn solve(&self, year: u32, day: &str, datafile: &str, mut parser: Box<dyn Solver>) {
        let filename = format!("{}/{year}/{day}/{datafile}", self.inputs);
        if !Path::new(&filename).exists() {
            warn!("{year}/{day} | missing file: {datafile}");
            return;
        }
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
        info!(
            "{year}/{day} | {datafile} solved in {:.3}sec",
            t1.duration_since(t0).unwrap().as_secs_f32()
        );
        if result.is_none() {
            warn!("{year}/{day} | {datafile} | no result returned?");
            return;
        }
        let (part1, part2) = result.unwrap();
        self.verify_result(year, day, datafile, 1, part1, expected_part_1);
        self.verify_result(year, day, datafile, 2, part2, expected_part_2);
    }

    fn verify_result(
        &self,
        year: u32,
        day: &str,
        datafile: &str,
        part: u32,
        result: String,
        value: Option<String>,
    ) {
        if let Some(expected) = value {
            if result == expected {
                info!("{year}/{day} | {datafile} | RESULT PART {part} - correct: {expected}")
            } else {
                error!("{year}/{day} | {datafile} | RESULT PART {part} - expected {expected}, actual {result}");
            }
        }
    }
}

fn init_logging(debug: bool) {
    let level = match debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} | {m}{n}")))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    log4rs::init_config(config).unwrap();
}
