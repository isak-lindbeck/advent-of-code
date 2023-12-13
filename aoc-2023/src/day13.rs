pub fn run(input: String) -> (usize, usize) {
    let mut ans_1 = 0;
    let mut ans_2 = 0;
    input.split("\n\n").for_each(|pattern| {
        let height = pattern.lines().count();
        let width = pattern.lines().next().unwrap().len();
        let mut map: Vec<Vec<char>> = vec![vec!['.'; height]; width];
        let mut rotated_map: Vec<Vec<char>> = vec![vec!['.'; width]; height];

        pattern.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                map[x][height - y - 1] = c;
                rotated_map[height - y - 1][x] = c
            });
        });

        let spaces = width - 1;
        for x in 0..spaces {
            let mut errors: usize = 0;
            (0..=x).rev().zip(x + 1..=spaces).for_each(|(l1, l2)| {
                let i: usize = map[l1].iter().zip(map[l2].iter()).filter(|(a, b)| a != b).count();
                errors += i;
            });
            if errors == 0 {
                ans_1 += x + 1;
            }
            if errors == 1 {
                ans_2 += x + 1;
            }
        }

        let spaces = height - 1;
        for y in 0..spaces {
            let mut errors: usize = 0;
            (0..=y).rev().zip(y + 1..=spaces).for_each(|(y1, y2)| {
                let i: usize = rotated_map[y1].iter().zip(rotated_map[y2].iter()).filter(|(a, b)| a != b).count();
                errors += i;
            });
            if errors == 0 {
                ans_1 += (height - y - 1) * 100;
            }
            if errors == 1 {
                ans_2 += (height - y - 1) * 100;
            }
        }
    });

    (ans_1, ans_2)
}