use std::cmp;
use std::collections::HashMap;

pub fn run(input: String) -> (usize, usize) {
    let input = clean(input);

    let lines: Vec<&str> = input.lines().collect();
    let mut gear_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let mut ans_1 = 0;
    for (line_idx, line) in lines.iter().enumerate() {
        let prev_line_idx = line_idx.checked_sub(1).unwrap_or(0);
        let next_line_idx = cmp::min(line_idx + 1, lines.len() - 1);
        let nearby_lines = lines.get(prev_line_idx..=next_line_idx).unwrap();

        let mut start_idx = 0;
        let mut in_number = false;
        for (char_idx, c) in line.chars().enumerate() {
            let is_digit = c.is_digit(10);
            let entering_number = is_digit && !in_number;
            let exiting_number = !is_digit && in_number;
            in_number = is_digit;

            if entering_number {
                start_idx = char_idx;
            } else if exiting_number {
                let end_idx = char_idx;
                let number = line.get(start_idx..end_idx).unwrap().parse::<i32>().unwrap();

                let from_idx = start_idx.checked_sub(1).unwrap_or(0);
                let to_idx = cmp::min(end_idx + 1, line.len());
                let is_part = nearby_lines.iter()
                    .map(|x| x.get(from_idx..to_idx).unwrap())
                    .any(|x| x.contains("#") || x.contains("*"));

                if is_part {
                    ans_1 += number;
                }

                nearby_lines.iter().zip(prev_line_idx..)
                    .for_each(|(nearby_line, line_idx)| {
                        nearby_line.chars().enumerate()
                            .filter(|(char_idx, _)| *char_idx >= from_idx)
                            .filter(|(char_idx, _)| *char_idx < to_idx)
                            .filter(|(_, c)| *c == '*')
                            .for_each(|(char_idx, _)| {
                                gear_map.entry((line_idx, char_idx))
                                    .and_modify(|x2| x2.push(number))
                                    .or_insert(vec![number]);
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

fn clean(input: String) -> String {
    input
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
        .replace("\n", " \n")
}
