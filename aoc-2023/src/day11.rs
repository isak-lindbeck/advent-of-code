use std::cmp;
use std::collections::HashSet;

use itertools::Itertools;

pub fn run(input: String) -> (usize, usize) {
    let length = input.lines().next().unwrap().len();

    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    let mut empty_x: HashSet<usize> = (0..length).collect();
    let mut empty_y: HashSet<usize> = (0..length).collect();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                galaxies.insert((x, y));
                empty_x.remove(&x);
                empty_y.remove(&y);
            }
        })
    });

    let mut ans_1 = 0;
    let mut ans_2 = 0;
    galaxies.iter().combinations(2).for_each(|pair|{
        let min_x = cmp::min(pair[0].0, pair[1].0);
        let max_x = cmp::max(pair[0].0, pair[1].0);
        let min_y = cmp::min(pair[0].1, pair[1].1);
        let max_y = cmp::max(pair[0].1, pair[1].1);

        let x_range = min_x..max_x;
        let y_range = min_y..max_y;

        let passed_empty_x = empty_x.iter().filter(|x| x_range.contains(x)).count();
        let passed_empty_y = empty_y.iter().filter(|y| y_range.contains(y)).count();
        let empty_distance = passed_empty_x + passed_empty_y;

        let distance = x_range.count() + y_range.count();
        ans_1 += distance + empty_distance;
        ans_2 += distance + empty_distance * (1_000_000 - 1);
    });

    (ans_1, ans_2)
}