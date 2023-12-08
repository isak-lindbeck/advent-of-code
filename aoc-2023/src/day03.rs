use std::cmp;
use std::collections::HashMap;

pub fn run(input: String) -> (usize, usize) {
    let input = input
        // .replace("*", "#")
        .replace("@", "#")
        .replace("$", "#")
        .replace("+", "#")
        .replace("=", "#")
        .replace("%", "#")
        .replace("#", "#")
        .replace("&", "#")
        .replace("-", "#")
        .replace("/", "#")
        .replace(".", " ")
        .replace("\n", " \n");

    let lines: Vec<&str> = input
        .lines()
        .collect();

    let mut gear_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let mut ans_1 = 0;
    for (line, line_idx) in lines.iter().zip(0..) {
        let len = line.len();
        let mut start_num_idx = 0;

        let prev_line_idx = cmp::max(1, line_idx) - 1;
        let next_line_idx = cmp::min(line_idx + 1, lines.len() - 1);
        let nearby_lines = lines.get(prev_line_idx..=next_line_idx).unwrap();

        let mut in_number = false;
        for (char_idx, c) in line.chars().enumerate() {
            let is_digit = c.is_digit(10);
            let entering_number = is_digit && !in_number;
            let exiting_number = !is_digit && in_number;
            in_number = is_digit;

            if entering_number {
                start_num_idx = char_idx;
            } else if exiting_number {
                let end_num_idx = char_idx;
                let num_int = line.get(start_num_idx..end_num_idx).unwrap().parse::<i32>().unwrap();

                let start_char_idx = cmp::max(start_num_idx, 1) - 1;
                let stop_char_idx = cmp::min(end_num_idx + 1, len);
                let is_part_number = nearby_lines.iter()
                    .map(|x| x.get(start_char_idx..stop_char_idx).unwrap())
                    .any(|x| x.contains("#") || x.contains("*"));

                if is_part_number {
                    ans_1 += num_int;
                }

                nearby_lines.iter().zip(prev_line_idx..)
                    .for_each(|(nearby_line, line_idx)| {
                        nearby_line.chars().enumerate()
                            .filter(|(char_idx, _)| *char_idx >= start_char_idx)
                            .filter(|(char_idx, _)| *char_idx < stop_char_idx)
                            .filter(|(_, c)| *c == '*')
                            .for_each(|(char_idx, _)| {
                                gear_map.entry((line_idx, char_idx))
                                    .and_modify(|x2| x2.push(num_int))
                                    .or_insert(vec![num_int]);
                            });
                    });
            }
        }
    }

    let ans_2: i32 = gear_map.values()
        .filter(|vec| vec.len() == 2)
        .map(|vec| vec[0] * vec[1])
        .sum();

    (ans_1 as usize, ans_2 as usize)
}
