pub fn run(input: String) -> (usize, usize) {
    let input = input;

    let input = clean(input);

    let len = input.lines().next().unwrap().len();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; len]; len];

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| map[x][len - 1 - y] = c);
    });

    let start: Coord = find_start_coord(&mut map);
    let start_char = get_start_tile_type(&map, &start);
    map[start.x as usize][start.y as usize] = start_char;

    let mut part_of_loop: Vec<Vec<bool>> = vec![vec![false; len]; len];
    part_of_loop[start.x as usize][start.y as usize] = true;

    let mut prev = start;
    let mut current = get_first_step(start, start_char);

    let mut ans_1: usize = 1;
    while current != start {
        part_of_loop[current.x as usize][current.y as usize] = true;
        ans_1 += 1;

        let dx: i32 = current.x - prev.x;
        let dy: i32 = current.y - prev.y;
        prev.x = current.x;
        prev.y = current.y;

        match &map[current.x as usize][current.y as usize] {
            '┌' => {
                current.x = current.x + dy;
                current.y = current.y + dx
            }
            '└' => {
                current.x = current.x - dy;
                current.y = current.y - dx
            }
            '─' => {
                current.x = current.x + dx;
                current.y = current.y + dy
            }
            '┐' => {
                current.x = current.x - dy;
                current.y = current.y - dx
            }
            '┘' => {
                current.x = current.x + dy;
                current.y = current.y + dx
            }
            '│' => {
                current.x = current.x + dx;
                current.y = current.y + dy
            }
            _ => panic!("Unknown char {}", &map[current.x as usize][current.y as usize]),
        };
    }
    let ans_1 = ans_1 / 2;

    let mut ans_2 = 0;
    for y in 0..len {
        let mut loop_passings = 0;
        let mut from_below = false;
        for x in 0..len {
            let on_loop = part_of_loop[x][y];
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

#[derive(PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

fn find_start_coord(map: &mut Vec<Vec<char>>) -> Coord {
    map.iter().enumerate().find_map(|(x, line)| {
        line.iter().enumerate().find_map(|(y, tile)| {
            if *tile == 'S' {
                Some(Coord { x: x as i32, y: y as i32 })
            } else {
                None
            }
        })
    }).unwrap()
}

fn get_start_tile_type(map: &Vec<Vec<char>>, start: &Coord) -> char {
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
    start_char
}

fn get_first_step(start: Coord, start_char: char) -> Coord {
    let first = match start_char {
        '┌' | '└' | '─' => Coord { x: start.x + 1, y: start.y },
        '┐' | '┘' => Coord { x: start.x - 1, y: start.y },
        _ => Coord { x: start.x, y: start.y + 1 },
    };
    first
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
