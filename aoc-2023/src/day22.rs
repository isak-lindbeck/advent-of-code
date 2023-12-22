use itertools::Itertools;
use crate::util::split_parse;

pub fn run(input: String) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut brick_lines: Vec<BrickLine> = input.lines().map(|l| {
        let (from, to) = l.split_once("~").unwrap();
        let from: Vec<usize> = split_parse(",", from);
        let to: Vec<usize> = split_parse(",", to);
        max_x = max_x.max(from[0]).max(to[0]);
        max_y = max_y.max(from[1]).max(to[1]);
        BrickLine {
            x_1: from[0],
            y_1: from[1],
            z_1: from[2],
            x_2: to[0],
            y_2: to[1],
            z_2: to[2]
        }
    })
        .sorted_by(|a, b| a.z_1.min(a.z_2).cmp(&b.z_1.min(b.z_2)))
        .collect();

    println!("max_x: {}, max_y: {}", max_x, max_y);

    let mut stack: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];

    for bick_line in brick_lines.iter_mut() {
        let mut max_z = 0;
        for x in bick_line.x_1..=bick_line.x_2 {
            for y in bick_line.y_1..=bick_line.y_2 {
                max_z = max_z.max(stack[x][y]);
            }
        }
        let drop = bick_line.z_1 - (max_z + 1);
        max_z += bick_line.z_2 - bick_line.z_1 + 1;

        for x in bick_line.x_1..=bick_line.x_2 {
            for y in bick_line.y_1..=bick_line.y_2 {
                stack[x][y] = max_z;
            }
        }
        bick_line.z_1 = bick_line.z_1 - drop;
        bick_line.z_2 = bick_line.z_2 - drop;
    }

    let mut critical_brick_lines : Vec<bool> = vec![false; brick_lines.len()];

    for brick_line in &brick_lines {
        let supporting_bl: Vec<usize> = brick_lines.iter().enumerate()
            .filter(|(_, b)| b.z_2 + 1 == brick_line.z_1)
            .filter(|(_, b)| {
                let share_x = brick_line.x_1 <= b.x_2 && b.x_1 <= brick_line.x_2;
                let share_y = brick_line.y_1 <= b.y_2 && b.y_1 <= brick_line.y_2;
                share_x && share_y
            })
            .map(|(i, _)| i)
            .collect();

        if supporting_bl.len() == 1 {
            critical_brick_lines[supporting_bl[0]] = true;
        }
    }

    let ans_1 = critical_brick_lines.iter().filter(|critical| !(**critical)).count();
    (ans_1, 0)
}

#[derive(Debug)]
struct BrickLine {
    x_1: usize,
    y_1: usize,
    z_1: usize,
    x_2: usize,
    y_2: usize,
    z_2: usize,
}