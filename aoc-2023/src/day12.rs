use std::{cmp};

use crate::util::split_parse;

pub fn run(input: String) -> (usize, usize) {
    // let ans_1: usize = input.lines().map(|line| {
    //     let split: Vec<_> = line.split(" ").collect();
    //     let row = split[0];
    //     let seq: Vec<usize> = split_parse(",", split[1]);
    //
    //     do_the_thing(&row, &seq[0..])
    // }).sum();

    // println!("Ans 1: {ans_1}");

    let ans_2: usize = input.lines().map(|line| {
        let split: Vec<_> = line.split(" ").collect();
        let row: &str = split[0];
        let seq: Vec<usize> = split_parse(",", split[1]);

        let e_row = [row, row, row, row, row].join("?");
        let e_seq: Vec<usize> = seq.iter().cycle().take(seq.len() * 5).map(|x| *x).collect();
        let b = do_the_thing(&e_row[0..], &e_seq[0..]);

        b
    }).sum();

    (0, ans_2)
}

fn do_the_thing(row: &str, seq: &[usize]) -> usize {
    if seq.len() == 0 {
        if row.contains('#') {
            return 0;
        }
        return 1;
    }
    let x: usize = seq.iter().sum();
    let sequence_size: usize = x + seq.len() - 1;
    let len = row.len();
    if len < sequence_size {
        return 0;
    }
    let next: char = row.chars().next().unwrap();
    if next == '.' {
        return do_the_thing(&row[1..], seq);
    }
    let mut res = 0;
    let next_size = seq[0];

    if next == '?' {
        res += do_the_thing(&row[1..], seq);
    }

    let option: Option<char> = row.chars().nth(next_size);
    let fits = row[0..next_size].chars().all(|c| c == '?' || c == '#')
        && (option.is_none() || option.unwrap() != '#');
    let idx = cmp::min(next_size + 1, len);
    if fits {
        res += do_the_thing(&row[idx..], &seq[1..]);
    }


    res
}

// fn do_the_other_thing2(row: &[char], seq: &[usize]) -> usize {
//     let mut c_pointer = 0;
//     let mut s_pointer = 0;
//     while c_pointer < row.len() && s_pointer < seq.len() {
//         let c = row[c_pointer];
//         let s = seq[s_pointer];
//
//         if c == '.' {
//             c_pointer += 1;
//         }
//
//         if c == '#' {
//             c_pointer += s;
//             s_pointer += 1;
//         }
//
//         if c == '?' {
//             c_pointer += 1;
//             // do_the_other_thing2(&row[c_pointer..], &seq[s_pointer..]);
//         }
//     }
//
//     0
// }

// fn do_the_thing_3(row: &str, seq: &[usize]) -> usize {
//     if seq.len() == 0 {
//         if row.contains('#') {
//             return 0;
//         }
//         return 1;
//     }
//
//     if row.len() == 0 {
//         return 0;
//     }
//
//     let next: char = row.chars().next().unwrap();
//     if next == '.' {
//         return do_the_thing_3(&row[1..], seq);
//     }
//
//     let next_size = seq[0];
//     let sum_size: usize = seq.iter().skip(1).sum();
//     let left_to_process = sum_size + seq.len() - 1;
//     let x = row.len().checked_sub(left_to_process + next_size).unwrap_or(0);
//
//     let mut sum = 0;
//     for range in 0..x {
//         let option1: Option<char> = row.chars().nth(range + next_size);
//         let option2: Option<char> = row.chars().nth(range.checked_sub(1).unwrap_or(usize::MAX));
//         let fits = row[range..range + next_size].chars().all(|c| c == '?' || c == '#')
//             && (option1.is_none() || option1.unwrap() != '#')
//             && (option2.is_none() || option2.unwrap() != '#');
//
//         if fits {
//             let from = cmp::min(range + next_size + 1, row.len());
//             // println!("{row} {:?} fits! {} -> {}", seq, range, range + next_size);
//             sum += do_the_thing_3(&row[from..], &seq[1..]);
//         }
//     }
//     sum
// }