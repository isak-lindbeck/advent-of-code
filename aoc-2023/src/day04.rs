use regex_macro::regex;

use crate::util::split_parse;

pub fn run(input: String) -> (usize, usize) {
    let input = input;
    let input = regex!(r" +").replace_all(&input, " ");
    let input = regex!(r"Card +\d+: +").replace_all(&input, "");

    let mut scrat_counts: Vec<usize> = vec![1; input.lines().count()];
    let mut ans_1 = 0;
    for (line_idx, line) in input.lines().enumerate() {
        let split: Vec<&str> = line.split(" | ").collect();
        let drawn_numbers: Vec<usize> = split_parse(" ", split[1]);
        let winners = split_parse(" ", split[0]).iter()
            .filter(|n| drawn_numbers.contains(n))
            .count();

        if winners > 0 {
            ans_1 += 2_usize.pow((winners - 1) as u32);
        }

        for scratch_to_update in line_idx + 1..=line_idx + winners {
            scrat_counts[scratch_to_update] += scrat_counts[line_idx];
        }
    }

    let ans_2: usize = scrat_counts.iter().sum();

    (ans_1, ans_2)
}