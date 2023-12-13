use std::cmp;
use std::collections::HashMap;

use crate::util::split_parse;

pub fn run(input: String) -> (usize, usize) {
    let ans_1: usize = input.lines().map(|line| {
        let split: Vec<_> = line.split(" ").collect();
        let row = split[0];
        let seq: Vec<usize> = split_parse(",", split[1]);

        do_the_thing(&mut HashMap::new(), &row, &seq[0..])
    }).sum();

    let ans_2: usize = input.lines().map(|line| {
        let split: Vec<_> = line.split(" ").collect();
        let row: &str = split[0];
        let seq: Vec<usize> = split_parse(",", split[1]);

        let e_row = [row, row, row, row, row].join("?");
        let e_seq: Vec<usize> = seq.iter().cycle().take(seq.len() * 5).map(|x| *x).collect();
        let b = do_the_thing(&mut HashMap::new(), &e_row[0..], &e_seq[0..]);

        b
    }).sum();

    (ans_1, ans_2)
}

fn do_the_thing<'a>(cache: &mut HashMap<(&'a str, &'a [usize]), usize>, row: &'a str, seq: &'a [usize]) -> usize {
    let key = (row, seq);
    let cached = cache.get(&key);
    if cached.is_some() {
        return *cached.unwrap();
    }

    if seq.len() == 0 {
        if row.contains('#') {
            cache.insert(key, 0);
            return 0;
        }
        cache.insert(key, 1);
        return 1;
    }
    let x: usize = seq.iter().sum();
    let sequence_size: usize = x + seq.len() - 1;
    let len = row.len();
    if len < sequence_size {
        cache.insert(key, 0);
        return 0;
    }
    let next: char = row.chars().next().unwrap();
    if next == '.' {
        let i = do_the_thing(cache, &row[1..], seq);
        cache.insert(key, i);
        return i;
    }
    let mut res = 0;

    if next == '?' {
        res += do_the_thing(cache, &row[1..], seq);
    }

    let next_size = seq[0];
    let option: Option<char> = row.chars().nth(next_size);
    let fits = row[0..next_size].chars().all(|c| c == '?' || c == '#')
        && (option.is_none() || option.unwrap() != '#');
    let idx = cmp::min(next_size + 1, len);
    if fits {
        res += do_the_thing(cache, &row[idx..], &seq[1..]);
    }

    cache.insert(key, res);
    res
}