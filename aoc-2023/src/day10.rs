use std::collections::HashSet;

use crate::day10::Pipe::*;

pub fn run(input: String) -> (usize, usize) {
    let input = input;

    let len = input.lines().next().unwrap().len();
    let mut map: Vec<Vec<Pipe>> = vec![vec![Ground; len]; len];

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let p = match c {
                '|' => UpToDown,
                '-' => LeftToRight,
                'F' => DownToRight,
                'L' => UpToRight,
                '7' => DownToLeft,
                'J' => UpToLeft,
                '.' => Ground,
                'S' => Start,
                _ => panic!("Unknown pipe! {}", c)
            };
            map[x][len - 1 - y] = p;
        });
    });

    let start: Coord = map.iter().enumerate().find_map(|(x, line)| {
        line.iter().enumerate().find_map(|(y, pipe)| {
            if *pipe == Start {
                Some(Coord { x: x as i32, y: y as i32 })
            } else {
                None
            }
        })
    }).unwrap();

    map[start.x as usize][start.y as usize] = LeftToRight;

    let mut prev = start;
    let mut current = Coord { x: start.x + 1, y: start.y }; // TODO fix
    let mut ans_1: usize = 1;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((start.x, start.y));
    while current != start {
        visited.insert((current.x, current.y));
        ans_1 += 1;
        let p = &map[current.x as usize][current.y as usize];
        let next = p.follow(prev, current);
        prev = current;
        current = next;
    }
    let ans_1 = ans_1 / 2;

    let mut ans_2 = 0;
    for y in 0..len {
        let y = len - 1 - y;
        for x in 0..len {
            let part_of_loop = visited.get(&(x as i32, y as i32)).is_some();
            if !part_of_loop {
                let mut value: i32 = 0;
                let mut flag = Ground;
                for dx in 1..=x {
                    let xx = x - dx;
                    let passed_tile = &map[xx][y];
                    let part_of_loop = visited.get(&(xx as i32, y as i32)).is_some();
                    if part_of_loop {
                        match passed_tile {
                            DownToRight => {
                                if flag == UpToLeft {
                                    value += 1;
                                }
                            }
                            UpToRight => {
                                if flag == DownToLeft {
                                    value += 1;
                                }
                            }
                            DownToLeft => {
                                flag = DownToLeft;
                            }
                            UpToLeft => {
                                flag = UpToLeft;
                            }
                            UpToDown => value += 1,
                            LeftToRight => {}
                            Start => {}
                            Ground => {}
                        };
                    }
                }
                let inside = value % 2 == 1;

                if inside {
                    ans_2 += 1;
                }
            }
        }
    }

    (ans_1, ans_2)
}

#[derive(PartialEq, Clone)]
enum Pipe {
    DownToRight,
    UpToRight,
    LeftToRight,
    DownToLeft,
    UpToLeft,
    UpToDown,
    Start,
    Ground,
}

#[derive(PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Pipe {
    fn follow(&self, prev_coord: Coord, this_coord: Coord) -> Coord {
        let dx: i32 = this_coord.x - prev_coord.x;
        let dy: i32 = this_coord.y - prev_coord.y;
        match self {
            DownToRight => Coord { x: this_coord.x + dy, y: this_coord.y + dx },
            UpToRight => Coord { x: this_coord.x - dy, y: this_coord.y - dx },
            LeftToRight => Coord { x: this_coord.x + dx, y: this_coord.y + dy },
            DownToLeft => Coord { x: this_coord.x - dy, y: this_coord.y - dx },
            UpToLeft => Coord { x: this_coord.x + dy, y: this_coord.y + dx },
            UpToDown => Coord { x: this_coord.x + dx, y: this_coord.y + dy },
            Start => this_coord,
            Ground => this_coord,
        }
    }
}