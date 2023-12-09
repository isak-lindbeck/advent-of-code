use itertools::Itertools;

use crate::util::split_parse;

pub fn run(input: String) -> (usize, usize) {
    let sequences: Vec<Vec<i64>> = input.lines().map(|line| split_parse(" ", line)).collect();

    let (ans_2, ans_1): (i64, i64) = sequences.iter()
        .map(calculate)
        .reduce(|acc, new| (acc.0 + new.0, acc.1 + new.1))
        .unwrap();

    (ans_1 as usize, ans_2 as usize)
}

fn calculate(v: &Vec<i64>) -> (i64, i64) {
    let differences: Vec<i64> = v.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if differences.iter().all_equal() {
        return (v[0] - differences[0], v[v.len() - 1] + differences[0]);
    }
    let res = calculate(&differences);
    (v[0] - res.0, v[v.len() - 1] + res.1)
}