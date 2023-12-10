use std::collections::HashSet;

pub fn run(input: String) -> (usize, usize) {
    let input = input;

    let input = clean(input);

    let len = input.lines().next().unwrap().len();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; len]; len];

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| map[x][len - 1 - y] = c);
    });

    let start: Coord = map.iter().enumerate().find_map(|(x, line)| {
        line.iter().enumerate().find_map(|(y, pipe)| {
            if *pipe == 'S' {
                Some(Coord { x: x as i32, y: y as i32 })
            } else {
                None
            }
        })
    }).unwrap();

    let left = map[(start.x - 1) as usize][start.y as usize];
    let right = map[(start.x + 1) as usize][start.y as usize];
    let up = map[start.x as usize][(start.y + 1) as usize];
    let left: bool = left == '─' || left == '└' || left == '┌';
    let right: bool = right == '─' || right == '┐' || right == '┘';
    let up: bool = up == '│' || up == '┐' || up == '┘';

    let start_char = match (left, right, up) {
        (false, true, false) => '┌',
        (false, true, true) => '└',
        (true, true, false) => '─',
        (true, false, false) => '┐',
        (true, false, true) => '┘',
        (false, false, true) => '│',
        _ => panic!("Unknown start char"),
    };

    map[start.x as usize][start.y as usize] = start_char;
    let mut prev = start;
    let mut current = match start_char {
        '┌' | '└' | '─' => Coord { x: start.x + 1, y: start.y },
        '┐' | '┘' => Coord { x: start.x - 1, y: start.y },
        _ => Coord { x: start.x, y: start.y + 1 },
    };

    let mut ans_1: usize = 1;
    let mut part_of_loop: HashSet<(i32, i32)> = HashSet::new();
    part_of_loop.insert((start.x, start.y));
    while current != start {
        part_of_loop.insert((current.x, current.y));
        ans_1 += 1;
        let p = &map[current.x as usize][current.y as usize];

        let dx: i32 = current.x - prev.x;
        let dy: i32 = current.y - prev.y;
        let next = match p {
            '┌' => Coord { x: current.x + dy, y: current.y + dx },
            '└' => Coord { x: current.x - dy, y: current.y - dx },
            '─' => Coord { x: current.x + dx, y: current.y + dy },
            '┐' => Coord { x: current.x - dy, y: current.y - dx },
            '┘' => Coord { x: current.x + dy, y: current.y + dx },
            '│' => Coord { x: current.x + dx, y: current.y + dy },
            _ => current,
        };

        prev = current;
        current = next;
    }
    let ans_1 = ans_1 / 2;

    let mut ans_2 = 0;
    for y in 0..len {
        let mut loop_passings: i32 = 0;
        let mut from_below = false;
        for x in 0..len {
            let on_loop = part_of_loop.get(&(x as i32, y as i32)).is_some();

            if on_loop {
                match &map[x][y] {
                    '┐' if !from_below => loop_passings = loop_passings + 1,
                    '┘' if from_below => loop_passings = loop_passings + 1,
                    '┌' => from_below = true,
                    '└' => from_below = false,
                    '│' => loop_passings += 1,
                    _ => {}
                }
            } else {
                if loop_passings % 2 == 1 {
                    ans_2 += 1;
                }
            }
        }
    }

    (ans_1, ans_2)
}

fn clean(input: String) -> String {
    let input = input
        .replace("F", "┌")
        .replace("L", "└")
        .replace("-", "─")
        .replace("7", "┐")
        .replace("J", "┘")
        .replace("|", "│");
    input
}

#[derive(PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}