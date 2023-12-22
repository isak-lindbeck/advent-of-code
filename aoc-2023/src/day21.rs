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
        let map_clone = map.clone();
        for y in 0..side {
            for x in 0..side {
                if map[x][y] != '#' {
                    let mut neighbours: Vec<char> = Vec::new();
                    if x != 0 {
                        neighbours.push(map_clone[x - 1][y])
                    }
                    if x < side - 1 {
                        neighbours.push(map_clone[x + 1][y])
                    }
                    if y != 0 {
                        neighbours.push(map_clone[x][y - 1])
                    }
                    if y < side - 1 {
                        neighbours.push(map_clone[x][y + 1])
                    }
                    if neighbours.contains(&'O') {
                        map[x][y] = 'O'
                    } else {
                        map[x][y] = '.';
                    }
                }
            }
        }
    }

    let mut ans_1 = 0;
    for y in 0..side {
        for x in 0..side {
            if map[x][y] == 'O' {
                ans_1 += 1;
            }
        }
    }

    (ans_1, 0)
}