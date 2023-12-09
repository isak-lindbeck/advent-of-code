use itertools::Itertools;

use crate::util::split_parse;

pub fn run(input: String) -> (usize, usize) {
    // let sequences: Vec<Vec<i64>> = input.lines().map(|l| split_parse(" ", l)).collect();

    let ans_1: i64 = input.lines().map(|l| split_parse(" ", l))
        .map(|v: Vec<i64>| {
            calculate_forwards(v)
        }).sum();


    let ans_2: i64 =  input.lines().map(|l| split_parse(" ", l))
        .map(|v: Vec<i64>| {
            calculate_backwards(v)
        }).sum();

    (ans_1 as usize, ans_2 as usize)
}

fn calculate_forwards(v: Vec<i64>) -> i64 {
    let differences: Vec<i64> = v.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if differences.iter().all_equal() {
        let next = v[v.len() - 1] + differences[0];
        next
    } else {
        let next = v[v.len() - 1] + calculate_forwards(differences);
        next
    }
}

fn calculate_backwards(v: Vec<i64>) -> i64 {
    let differences: Vec<i64> = v.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if differences.iter().all_equal() {
        let prev = v[0] - differences[0];
        prev
    } else {
        let prev = v[0] - calculate_backwards(differences);
        prev
    }
}