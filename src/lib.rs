const MAX_DAY: u8 = 6;

pub mod args;
use crate::args::Args;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
// pub mod day07;
// pub mod day08;
// pub mod day09;
// pub mod day10;
// pub mod day11;
// pub mod day12;

pub struct RunResult {
    pub day: u8,
    pub answer1: Option<usize>,
    pub answer2: Option<usize>
}

impl RunResult {
    pub fn print(self) -> () {
        println!("Results for Day {}", self.day);

        match self.answer1 {
            Some(ans) => println!("Part 1: {}", ans),
            None => println!("No answer to Part 1 yet")
        };

        match self.answer2 {
            Some(ans) => println!("Part 2: {}", ans),
            None => println!("No answer to Part 2 yet")
        };

    }
}

pub fn run(args: Args) -> Vec<RunResult> {

    if let Some(day) = args.day {
        vec![run_day(day)]
    } else {
        run_all()
    }
}

fn run_day(day: u8) -> RunResult {
    let run = match day {
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        5 => day05::run,
        6 => day06::run,
        // 7 => day07::run,
        // 8 => day08::run,
        // 9 => day09::run,
        // 10 => day10::run,
        // 11 => day11::run,
        // 12 => day12::run,
        _ => panic!("There is no day {day} in Advent of Code")
    };

    let (answer1, answer2) = run();

    RunResult {
        day,
        answer1,
        answer2
    }

}

fn run_all() -> Vec<RunResult> {
    (1..=MAX_DAY)
        .map(|day| run_day(day))
        .collect()
}
