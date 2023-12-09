use itertools::Itertools;

use crate::util::split_parse;

pub fn run(input: String) -> (usize, usize) {
    let sequences: Vec<Vec<i64>> = input.lines().map(|line| split_parse(" ", line)).collect();

    let ans_1: i64 = sequences.iter()
        .map(calculate_forwards)
        .sum();


    let ans_2: i64 = sequences.iter()
        .map(calculate_backwards)
        .sum();

    (ans_1 as usize, ans_2 as usize)
}

fn calculate_forwards(v: &Vec<i64>) -> i64 {
    let differences: Vec<i64> = v.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if differences.iter().all_equal() {
        return v[v.len() - 1] + differences[0];
    }
    v[v.len() - 1] + calculate_forwards(&differences)
}

fn calculate_backwards(v: &Vec<i64>) -> i64 {
    let differences: Vec<i64> = v.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if differences.iter().all_equal() {
        return v[0] - differences[0];
    }
    v[0] - calculate_backwards(&differences)
}