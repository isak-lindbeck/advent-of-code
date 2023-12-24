use itertools::Itertools;

use crate::util::split_parse;

pub fn run(input: String) -> (usize, usize) {
    let min: f64 = 200000000000000.0;
    let max: f64 = 400000000000000.0;

    let lines: Vec<Line> = input.lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let p: Vec<f64> = split_parse(", ", p);
            let v: Vec<f64> = split_parse(", ", v);
            let x = p[0];
            let dx = v[0];
            let y = p[1];
            let dy = v[1];

            let m = dy / dx;
            let b = y - (m * x);
            Line { x, dx, y, dy, m, b }
        }).collect();

    let mut ans_1 = 0;
    lines.iter().combinations(2).for_each(|l| {
        let x = (l[1].b - l[0].b) / (l[0].m - l[1].m);
        let y = (x * l[0].m) + l[0].b;

        if x >= min && x <= max && y >= min && y <= max {
            let is_future_x_0 = (l[0].dx > 0.0 && x >= l[0].x) || (l[0].dx < 0.0 && x <= l[0].x);
            let is_future_x_1 = (l[1].dx > 0.0 && x >= l[1].x) || (l[1].dx < 0.0 && x <= l[1].x);
            let is_future_y_0 = (l[0].dy > 0.0 && y >= l[0].y) || (l[0].dy < 0.0 && y <= l[0].y);
            let is_future_y_1 = (l[1].dy > 0.0 && y >= l[1].y) || (l[1].dy < 0.0 && y <= l[1].y);
            if is_future_x_0 && is_future_x_1 && is_future_y_0 && is_future_y_1 {
                ans_1 += 1;
            }
        }
    });

    (ans_1, 0)
}

struct Line {
    x: f64,
    dx: f64,
    y: f64,
    dy: f64,
    m: f64,
    b: f64,
}