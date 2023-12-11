use std::collections::HashSet;

use itertools::Itertools;

pub fn run(input: String) -> (usize, usize) {

    let line_len = input.lines().next().unwrap().len();
    let empty_row = ".".repeat(line_len);
    let double_empty_row = format!("{}\n{}", empty_row, empty_row);
    let input = input.replace(empty_row.as_str(), &double_empty_row);

    let height = input.lines().count();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; height]; line_len];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                map[x][y] = '#';
            }
        })
    });
    let copy = map.clone();
    for (i, column) in copy.iter().enumerate().rev() {
        if column.iter().all_equal() {
            map.insert(i, vec!['.'; height]);
        }
    }

    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            print!("{}", map[x][y]);
            if map[x][y] == '#' {
                galaxies.insert((x, y));
            }
        }
        println!()
    }

    let ans_1: usize = galaxies.iter().combinations(2).map(|v| {
        let a = v[0];
        let b = v[1];
        let x3 = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
        println!("({} {}) -> ({} {}) == {}", a.0, a.1, b.0, b.1, x3);
        x3
    }).sum();

    (ans_1, 0)
}