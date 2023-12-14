pub fn run(input: String) -> (usize, usize) {

    let side = input.lines().count();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; side]; side];

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[x][y] = c;
        });
    });

    let mut ans_1 = 0;
    for x in 0..side {
        let mut sum = 0;
        for y in 0..side {
           match map[x][y] {
               '.' => sum += 1,
               '#' => sum = 0,
               'O' => {
                   ans_1 += side - y + sum;
               },
               _ => panic!("Unkown char")
           }
        }
    }


    for y in 0..side {
        for x in 0..side {
            print!("{} ", map[x][y]);
        }
        println!();
    }

    (ans_1,0)
}