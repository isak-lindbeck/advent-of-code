use std::ops::Div;

use num::integer::sqrt;
use regex_macro::regex;

use crate::util::split_parse;

pub fn run(input: String) -> (usize, usize) {
    let input = regex!(" +").replace_all(&input, " ");
    let input = regex!("Time: +").replace(&input, "");
    let input = regex!("Distance: +").replace(&input, "");

    let max_times: Vec<usize> = split_parse(" ", input.lines().next().unwrap());
    let distances_to_beat: Vec<usize> = split_parse(" ", input.lines().last().unwrap());

    let mut ans_1 = 1;
    for (max_wait_time, distance_to_beat) in max_times.iter().zip(distances_to_beat.iter()) {
        let winning_races = calculate_race(*max_wait_time, *distance_to_beat);
        if winning_races > 0 {
            ans_1 *= winning_races;
        }
    }

    let input = &input.replace(" ", "");
    let num: Vec<usize> = split_parse("\n", input);

    let max_wait_time = num[0];
    let distance_to_beat = num[1];
    let ans_2 = calculate_race(max_wait_time, distance_to_beat);

    (ans_1, ans_2)
}

fn calculate_race(max_wait_time: usize, distance_to_beat: usize) -> usize {
    let a: usize = (max_wait_time + sqrt(max_wait_time.pow(2) - 4 * distance_to_beat)).div(2);
    let b: usize = (max_wait_time - sqrt(max_wait_time.pow(2) - 4 * distance_to_beat)).div_ceil(2);
    a - b + 1
}