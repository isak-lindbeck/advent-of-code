extern crate core;

use std::{env, fs};
use std::collections::HashMap;
use std::time::SystemTime;

use colored::Colorize;
use regex_macro::regex;

mod util;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    let mut days = 1..=10;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day: i32 = args[1].parse().expect("Unexpected first argument, not a number");
        days = day..=day;
    }

    let answer_lookup = answers();

    let start = SystemTime::now();
    for day in days {
        let input = fs::read_to_string(format!("./data/day{:0>2}.txt", day)).expect("Could not read the file");
        let (ans_1, ans_2) = match day {
            1 => { day01::run(input) }
            2 => { day02::run(input) }
            3 => { day03::run(input) }
            4 => { day04::run(input) }
            5 => { day05::run(input) }
            6 => { day06::run(input) }
            7 => { day07::run(input) }
            8 => { day08::run(input) }
            9 => { day09::run(input) }
            10 => { day10::run(input) }
            _ => { panic!("Day {} is not implemented yet", day); }
        };
        // println!("Day {day}: {ans_1}, {ans_2}");

        match answer_lookup.get(&day) {
            Some((actual_1, actual_2)) if actual_1 == &ans_1 && actual_2 == &ans_2 => {
                println!("Day {day}: {ans_1}, {ans_2}")
            }
            Some((actual_1, actual_2)) => {
                println!("Day {day}: {ans_1}, {ans_2} {} expected: {actual_1}, {actual_2}!", "!!".red())
            }
            None => println!("Day {day}: {ans_1}, {ans_2} {} nothing to compare with", "??".blue()),
        }
    }

    let duration = start.elapsed().unwrap();
    println!("Execution time: {}s {}ms {}μs {}ns",
             duration.as_secs(),
             duration.subsec_millis(),
             duration.subsec_micros() - (duration.subsec_millis() * 1000),
             duration.subsec_nanos() - (duration.subsec_micros() * 1000)
    );
}

fn answers() -> HashMap<i32, (usize, usize)> {
    let answers = fs::read_to_string("./data/answers.txt").expect("Could not read the answer file");
    let answers = regex!(r"Day +\d+: ").replace_all(&answers, "");
    let mut ans_lookup: HashMap<i32, (usize, usize)> = HashMap::new();
    answers.lines().enumerate().for_each(|(idx, line)| {
        let ans: Vec<usize> = line.split(", ").map(|s| s.parse::<usize>().unwrap()).collect();
        ans_lookup.insert(1 + idx as i32, (ans[0], ans[1]));
    });
    ans_lookup
}
