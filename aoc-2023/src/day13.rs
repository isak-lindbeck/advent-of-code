pub fn run(input: String) -> (usize, usize) {
    let mut ans_1 = 0;
    let mut ans_2 = 0;
    input.split("\n\n").for_each(|pattern| {
        let height = pattern.lines().count();
        let width = pattern.lines().next().unwrap().len();
        let mut map: Vec<Vec<char>> = vec![vec!['.'; height]; width];
        let mut map_rotated: Vec<Vec<char>> = vec![vec!['.'; width]; height];

        pattern.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                map[x][y] = c;
                map_rotated[y][x] = c;
            });
        });

        let res_1: (usize, usize) = do_the_thing(width, height, &map);
        let res_2: (usize, usize) = do_the_thing(height, width, &map_rotated);
        ans_1 += res_1.0 + res_2.0 * 100;
        ans_2 += res_1.1 + res_2.1 * 100;
    });

    (ans_1, ans_2)
}

fn do_the_thing(width: usize, height: usize, map: &Vec<Vec<char>>) -> (usize, usize) {
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for x in 0..width - 1 {
        let mut errors: usize = 0;
        (0..=x).rev().zip(x + 1..width).for_each(|(x1, x2)| {
            let differences: usize = (0..height).filter(|y| map[x1][*y] != map[x2][*y]).count();
            errors += differences;
        });
        if errors == 0 {
            sum_1 += x + 1;
        }
        if errors == 1 {
            sum_2 += x + 1;
        }
    }
    (sum_1, sum_2)
}