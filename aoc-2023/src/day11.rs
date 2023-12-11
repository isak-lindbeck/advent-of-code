use std::cmp;
use std::collections::HashSet;

use itertools::Itertools;

pub fn run(input: String) -> (usize, usize) {

    let line_len = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; height]; line_len];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                map[x][y] = '#';
            }
        })
    });

    let mut empty_x: HashSet<usize> = HashSet::new();
    let mut empty_y: HashSet<usize> = HashSet::new();
    let empty_row = ".".repeat(line_len);
    input.lines().enumerate().filter(|(_, line)| empty_row.eq(line)).for_each(|(y, _)| {
       empty_y.insert(y);
    });
    map.iter().enumerate().filter(|(_, v)| v.iter().all_equal()).for_each(|(x, _)| {
        empty_x.insert(x);
    });

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

    println!("empty x: {:?}", empty_x);
    println!("empty y: {:?}", empty_y);

    let ans_1: usize = galaxies.iter().combinations(2).map(|v| {
        let x1 = v[0].0;
        let y1 = v[0].1;
        let x2 = v[1].0;
        let y2 = v[1].1;

        let empty_distance = 1_000_000 - 1;
        let passed_empty_x = empty_x.iter().filter(|x| (cmp::min(x1,x2)..cmp::max(x1,x2)).contains(x)).count() * empty_distance;
        let passed_empty_y = empty_y.iter().filter(|y| (cmp::min(y1,y2)..cmp::max(y1,y2)).contains(y)).count() * empty_distance;
        let empty_distance = passed_empty_x + passed_empty_y;
        let distance = x1.abs_diff(x2) + y1.abs_diff(y2) + empty_distance;
        println!("({} {}) -> ({} {}) + {}x + {}y == {}", x1, y1, x2, y2, passed_empty_x, passed_empty_y, distance);


        distance
    }).sum();

    (ans_1, 0)
}