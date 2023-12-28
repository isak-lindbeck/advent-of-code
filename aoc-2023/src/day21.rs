pub fn run(input: String) -> (usize, usize) {
    let side = input.lines().count();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; side]; side];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == 'S' {
                map[x][y] = 'O';
            } else {
                map[x][y] = c;
            }
        });
    });

    for _ in 0..64 {
        map = step(&map);
    }
    let ans_1 = count_full(&map);

    for _ in 0..65 {
        map = step(&map);
    }
    let full_even = count_full(&map);
    let corners_even = count_corners(&map);

    map = step(&map);
    let full_odd = count_full(&map);
    let corners_odd = count_corners(&map);

    let steps: usize = 26501365;
    let maps_to_cover = (steps - 65) / 131;
    let corner_even_count = maps_to_cover + 1;
    let corner_odd_count = maps_to_cover;
    let full_even_count = corner_even_count.pow(2);
    let full_odd_count = corner_odd_count.pow(2);

    let ans_2 = (full_even_count * full_even) + (full_odd_count * full_odd)
        + (corner_odd_count * corners_odd)
        - (corner_even_count * corners_even);

    (ans_1, ans_2)
}

fn step(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map_clone = map.clone();
    let side = map.len();
    for y in 0..side {
        for x in 0..side {
            if map[x][y] != '#' {
                if x != 0 && map[x - 1][y] == 'O' {
                    map_clone[x][y] = 'O';
                } else if x < side - 1 && map[x + 1][y] == 'O' {
                    map_clone[x][y] = 'O';
                } else if y != 0 && map[x][y - 1] == 'O' {
                    map_clone[x][y] = 'O';
                } else if y < side - 1 && map[x][y + 1] == 'O' {
                    map_clone[x][y] = 'O';
                } else {
                    map_clone[x][y] = '.';
                }
            }
        }
    }
    map_clone
}

fn count_full(map: &Vec<Vec<char>>) -> usize {
    let side = map.len();
    let mut sum = 0;
    for y in 0..side {
        for x in 0..side {
            if map[x][y] == 'O' {
                sum += 1;
            }
        }
    }
    sum
}

fn count_corners(map: &Vec<Vec<char>>) -> usize {
    let side = map.len();
    let mid = side / 2;
    let mut sum = 0;
    for y in 0..side {
        for x in 0..side {
            if x.abs_diff(mid) + y.abs_diff(mid) > mid {
                if map[x][y] == 'O' {
                    sum += 1;
                }
            }
        }
    }
    sum
}