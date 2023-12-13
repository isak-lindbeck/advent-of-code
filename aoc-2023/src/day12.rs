use std::cmp;
use std::collections::HashMap;

use crate::util::split_parse;

const UNKNOWN: u8 = b'?';
const WORKING: u8 = b'.';
const BROKEN: u8 = b'#';

pub fn run(input: String) -> (usize, usize) {
    let mut ans_1: usize = 0;
    let mut ans_2: usize = 0;
    input.lines().for_each(|line| {
        let split: Vec<_> = line.split(" ").collect();
        let row: &str = split[0];
        let seq: Vec<u8> = split_parse(",", split[1]);
        let data: Vec<u8> = row.chars().map(parse_state).collect();

        let expanded_row = [row, row, row, row, row].join("?");
        let expanded_seq: Vec<u8> = seq.iter().cycle().take(seq.len() * 5).map(|x| *x).collect();
        let expanded_data: Vec<u8> = expanded_row.chars().map(parse_state).collect();

        ans_1 += calculate(&mut HashMap::new(), 0, 0,&data[0..], &seq[0..]);
        ans_2 += calculate(&mut HashMap::new(), 0, 0,&expanded_data[0..], &expanded_seq[0..]);
    });

    (ans_1, ans_2)
}

fn calculate<'a>(
    cache: &mut HashMap<(usize, usize), usize>,
    row_counter: usize,
    seq_counter: usize,
    row: &'a [u8],
    seq: &'a [u8],
) -> usize {
    if seq.len() == 0 {
        if row.iter().any(|s| s == &BROKEN) {
            return 0;
        }
        return 1;
    }

    let sequence_sum: u8 = seq.iter().sum();
    let sequence_size: u8 = sequence_sum + (seq.len() as u8) - 1;
    if (row.len() as u8) < sequence_size {
        return 0;
    }

    let next: u8 = row[0];
    if next == WORKING {
        return calculate_cached(cache, row_counter + 1,seq_counter,&row[1..], seq);
    }

    let next_size = seq[0] as usize;
    let char_after: Option<&u8> = row.get(next_size);
    let sequence_fits = !row[0..next_size].iter().any(|c| c == &WORKING)
        && (char_after.is_none() || char_after.unwrap() != &BROKEN);
    let mut res = 0;
    if sequence_fits {
        let idx = cmp::min(next_size + 1, row.len());
        res += calculate_cached(cache, row_counter + idx,seq_counter + 1,&row[idx..], &seq[1..]);
    }
    if next == UNKNOWN {
        res += calculate_cached(cache, row_counter + 1, seq_counter,&row[1..], seq);
    }
    return res;
}

fn calculate_cached<'a>(
    cache: &mut HashMap<(usize, usize), usize>,
    row_counter: usize,
    seq_counter: usize,
    row: &'a [u8],
    seq: &'a [u8],
) -> usize {
    let cache_key = (row_counter, seq_counter);
    let cached = cache.get(&cache_key);
    if cached.is_some() {
        return *cached.unwrap();
    }
    let res = calculate(cache, row_counter, seq_counter, row, seq);
    cache.insert(cache_key, res);
    return res;
}

fn parse_state(c: char) -> u8 {
    match c {
        '?' => UNKNOWN,
        '.' => WORKING,
        '#' => BROKEN,
        _ => panic!("Unknown input: {c}")
    }
}
