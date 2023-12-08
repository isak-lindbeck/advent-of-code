use std::cmp;

static RED_MAX: usize = 12;
static GREEN_MAX: usize = 13;
static BLUE_MAX: usize = 14;

pub fn run(input: String) -> (usize, usize) {
    let mut ans_1: usize = 0;
    let mut ans_2: usize = 0;
    for line in input.lines() {
        let split: Vec<_> = line.split(": ").collect();

        let game_number: usize = split[0].replace("Game ", "").parse().unwrap();

        let mut red_max_drawn = 1;
        let mut green_max_drawn = 1;
        let mut blue_max_drawn = 1;

        for draw in split[1].split("; ") {
            for color_amount_draw in draw.split(", ") {
                let split: Vec<_> = color_amount_draw.split(" ").collect();
                let drawn_amount: usize = split[0].parse().unwrap();
                let drawn_color = split[1];

                match drawn_color {
                    "red" => red_max_drawn = cmp::max(red_max_drawn, drawn_amount),
                    "green" => green_max_drawn = cmp::max(green_max_drawn, drawn_amount),
                    "blue" => blue_max_drawn = cmp::max(blue_max_drawn, drawn_amount),
                    _ => panic!("Unexpected color: {}", drawn_color),
                };
            }
        }
        if red_max_drawn <= RED_MAX && green_max_drawn <= GREEN_MAX && blue_max_drawn <= BLUE_MAX {
            ans_1 += game_number;
        }

        ans_2 += red_max_drawn * green_max_drawn * blue_max_drawn;
    }

    (ans_1, ans_2)
}
