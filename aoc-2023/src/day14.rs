use std::collections::HashMap;

const ROUND: u8 = b'O';
const EMPTY: u8 = b'.';
const CUBE: u8 = b'#';

fn parse_tile(c: char) -> u8 {
    match c {
        'O' => ROUND,
        '.' => EMPTY,
        '#' => CUBE,
        _ => panic!("Unknown input: {c}")
    }
}

pub fn run(input: String) -> (usize, usize) {
    let side = input.lines().count();
    let mut map: Vec<Vec<u8>> = vec![vec![EMPTY; side]; side];

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[x][y] = parse_tile(c);
        });
    });

    tilt_north(&mut map);
    let ans_1 = calculate_load(&map);

    let mut cache: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();
    let mut loop_found = false;

    let mut cycle_count = 0;
    let goal = 1_000_000_000;
    while cycle_count < goal {
        tilt_north(&mut map);
        tilt_west(&mut map);
        tilt_south(&mut map);
        tilt_east(&mut map);

        let clone = map.clone();
        let option = cache.get(&clone);
        if option.is_some() && !loop_found {
            let loop_len = cycle_count - option.unwrap();
            let left_to_process = (goal - cycle_count) % loop_len;
            cycle_count = goal - left_to_process + 1;
            loop_found = true;
        } else {
            cache.insert(clone, cycle_count);
            cycle_count += 1;
        }
    }
    let ans_2 = calculate_load(&map);

    (ans_1, ans_2)
}

fn calculate_load(map: &Vec<Vec<u8>>) -> usize {
    let side = map.len();
    let mut sum = 0;
    for y in 0..side {
        for x in 0..side {
            if map[x][y] == ROUND {
                sum += side - y;
            }
        }
    }
    sum
}

fn tilt_north(map: &mut Vec<Vec<u8>>) {
    let side = map.len();
    for x in 0..side {
        let mut last_anchor = 0;
        for y in 0..=side {
            if y == side || map[x][y] == CUBE {
                let mut stone_count = 0;
                for i in last_anchor..y {
                    if map[x][i] == ROUND {
                        stone_count += 1;
                    }
                    map[x][i] = EMPTY;
                }
                for i in last_anchor..last_anchor + stone_count {
                    map[x][i] = ROUND;
                }
                last_anchor = y + 1;
            }
        }
    }
}

fn tilt_west(map: &mut Vec<Vec<u8>>) {
    let side = map.len();
    for y in 0..side {
        let mut last_anchor = 0;
        for x in 0..=side {
            if x == side || map[x][y] == CUBE {
                let mut stone_count = 0;
                for i in last_anchor..x {
                    if map[i][y] == ROUND {
                        stone_count += 1;
                    }
                    map[i][y] = EMPTY;
                }
                for i in last_anchor..last_anchor + stone_count {
                    map[i][y] = ROUND;
                }
                last_anchor = x + 1;
            }
        }
    }
}

fn tilt_south(map: &mut Vec<Vec<u8>>) {
    let side = map.len();
    for x in 0..side {
        let mut last_anchor = 0;
        for y in 0..=side {
            if y == side || map[x][y] == CUBE {
                let mut stone_count = 0;
                for i in last_anchor..y {
                    if map[x][i] == ROUND {
                        stone_count += 1;
                    }
                    map[x][i] = EMPTY;
                }
                for i in y - stone_count..y {
                    map[x][i] = ROUND;
                }
                last_anchor = y + 1;
            }
        }
    }
}

fn tilt_east(map: &mut Vec<Vec<u8>>) {
    let side = map.len();
    for y in 0..side {
        let mut last_anchor = 0;
        for x in 0..=side {
            if x == side || map[x][y] == CUBE {
                let mut stone_count = 0;
                for i in last_anchor..x {
                    if map[i][y] == ROUND {
                        stone_count += 1;
                    }
                    map[i][y] = EMPTY;
                }
                for i in x - stone_count..x {
                    map[i][y] = ROUND;
                }
                last_anchor = x + 1;
            }
        }
    }
}