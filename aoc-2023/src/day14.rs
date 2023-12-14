use std::collections::HashMap;

const ROLLING: u8 = b'O';
const EMPTY: u8 = b'.';
const ANCHOR: u8 = b'#';

pub fn run(input: String) -> (usize, usize) {
    let mut map = parse_input(input);
    tilt_north(&mut map);
    let ans_1 = calculate_load(&map);

    let mut cache: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();
    let mut cycle_count = 0;
    let cycle_goal = 1_000_000_000;
    while cycle_count < cycle_goal {
        tilt_north(&mut map);
        tilt_west(&mut map);
        tilt_south(&mut map);
        tilt_east(&mut map);

        if let Some(prev_cycle) = cache.get(&map) {
            let loop_len = cycle_count - prev_cycle;
            let cycles_left = (cycle_goal - cycle_count) % loop_len;
            cycle_count = cycle_goal - cycles_left;
        } else {
            cache.insert(map.clone(), cycle_count);
        }
        cycle_count += 1;
    }
    let ans_2 = calculate_load(&map);

    (ans_1, ans_2)
}

fn calculate_load(map: &Vec<Vec<u8>>) -> usize {
    map.iter().flat_map(|v| v.iter().enumerate())
        .filter(|(_, c)| *c == &ROLLING)
        .map(|(y, _)| map.len() - y )
        .sum()
}

fn tilt_north(map: &mut Vec<Vec<u8>>) {
    let side = map.len();
    for x in 0..side {
        let mut last_free = 0;
        for y in 0..side {
            if map[x][y] == ANCHOR {
                last_free = y + 1;
            }
            if map[x][y] == ROLLING {
                map[x][y] = EMPTY;
                map[x][last_free] = ROLLING;
                last_free += 1;
            }
        }
    }
}

fn tilt_west(map: &mut Vec<Vec<u8>>) {
    let side = map.len();
    for y in 0..side {
        let mut last_free = 0;
        for x in 0..side {
            if map[x][y] == ANCHOR {
                last_free = x + 1;
            }
            if map[x][y] == ROLLING {
                map[x][y] = EMPTY;
                map[last_free][y] = ROLLING;
                last_free += 1;
            }
        }
    }
}

fn tilt_south(map: &mut Vec<Vec<u8>>) {
    let side = map.len();
    for x in 0..side {
        let mut last_stuck = side;
        for y in (0..side).rev() {
            if map[x][y] == ANCHOR {
                last_stuck = y;
            }
            if map[x][y] == ROLLING{
                map[x][y] = EMPTY;
                map[x][last_stuck - 1] = ROLLING;
                last_stuck -= 1;
            }
        }
    }
}

fn tilt_east(map: &mut Vec<Vec<u8>>) {
    let side = map.len();
    for y in 0..side {
        let mut last_stuck = side;
        for x in (0..side).rev() {
            if map[x][y] == ANCHOR{
                last_stuck = x;
            }
            if map[x][y] == ROLLING{
                map[x][y] = EMPTY;
                map[last_stuck - 1][y] = ROLLING;
                last_stuck -= 1;
            }
        }
    }
}

fn parse_input(input: String) -> Vec<Vec<u8>> {
    let side = input.lines().count();
    let mut map: Vec<Vec<u8>> = vec![vec![EMPTY; side]; side];

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[x][y] = parse_char(c);
        });
    });
    map
}

fn parse_char(c: char) -> u8 {
    match c {
        'O' => ROLLING,
        '.' => EMPTY,
        '#' => ANCHOR,
        _ => panic!("Unknown input: {c}")
    }
}
