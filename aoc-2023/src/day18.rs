use Direction::{Down, Left, Right, Up};

pub fn run(input: String) -> (usize, usize) {
    let instruction: Vec<(Direction, i64)> = input.lines().map(|line| {
        let split: Vec<_> = line.split(" ").collect();
        let dir = match split[0] {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("Unexpected direction: {}", split[0]),
        };
        let range = split[1].parse::<i64>().unwrap();

        let string = split[2].replace("(#", "").replace(")", "");
        let dir = match string.chars().last().unwrap() {
            '0' => Right,
            '1' => Down,
            '2' => Left,
            '3' => Up,
            _ => panic!("Unexpected direction: {}", string),
        };

        let range = i64::from_str_radix(&string[..string.len() - 1], 16).unwrap();

        println!("{}, {:?}", split[2], (dir, range));

        (dir, range)
    }).collect();

    let mut current: (i64, i64) = (0, 0);
    let mut ups_and_downs: Vec<(i64, Range)> = Vec::new();

    for i in instruction {
        let (d, r) = i;
        let (curr_x, curr_y) = current;
        let next = match d {
            Up => (curr_x, curr_y - r),
            Down => (curr_x, curr_y + r),
            Left => (curr_x - r, curr_y),
            Right => (curr_x + r, curr_y),
        };
        let (_, next_y) = next;
        match d {
            Up => ups_and_downs.push((curr_x, Range { direction: Up, from: next_y, to: curr_y })),
            Down => ups_and_downs.push((curr_x, Range { direction: Down, from: curr_y, to: next_y })),
            _ => {}
        }

        println!("{:?} -> {:?}", current, next);

        current = next;
    }

    ups_and_downs.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ans_1 = 0;

    let mut current_ranges: Vec<Range> = Vec::new();
    let mut current_sum: i64 = 0;

    let mut current_x = ups_and_downs[0].0;
    let start_range = &ups_and_downs[0].1;
    let direction: Direction = start_range.direction;

    let mut ups_and_downs: Vec<(i64, Range)> = ups_and_downs.iter().map(|(i, r)| {
        if r.direction == direction {
            (*i, *r)
        } else {
            (*i + 1, *r)
        }
    }).collect();
    ups_and_downs.sort_by(|a, b| a.0.cmp(&b.0).then(((a.1.direction == direction) as i64).cmp(&((b.1.direction == direction) as i64))));
    let last = ups_and_downs.iter().last().unwrap();
    ups_and_downs.push((last.0 + 1, Range { direction: Left, from: 0, to: 0}));
    for (x, range) in &ups_and_downs[0..] {
        if *x != current_x {
            let inc_count = current_ranges.iter().filter(|r| r.direction == direction).count() as i64;
            let increase: i64 = current_ranges.iter().filter(|r| r.direction == direction).map(|r| r.from.abs_diff(r.to) as i64).sum();
            let increase = increase + inc_count;
            let red_count = current_ranges.iter().filter(|r| r.direction != direction).count() as i64;
            let reduce: i64 = current_ranges.iter().filter(|r| r.direction != direction).map(|r| r.from.abs_diff(r.to) as i64).sum();
            let reduce = reduce - red_count;
            let next_sum = increase - reduce;


            let delta = current_x.abs_diff(*x) as i64;
            // let delta = current_x.abs_diff(*x) + (range.direction != direction) as u32;
            println!("X: {x}       {:?}", current_ranges);
            println!("VALUE INCREASE: {increase} -> REDUCE: {reduce}    Delta: {delta} x {next_sum} Add: {}", next_sum * delta);
            ans_1 += next_sum * delta;
            current_x = *x;
            println!();
            current_sum = next_sum;

            if range.direction == Left {
                break;
            }
        }

        let mut from = range.from;
        let mut to = range.to;
        if let Some(idx) = current_ranges.iter().position(|r| r.to == range.from && r.direction == range.direction) {
            let r = current_ranges.remove(idx);
            from = r.from;
        }
        if let Some(idx) = current_ranges.iter().position(|r| range.to == r.from && r.direction == range.direction) {
            let r = current_ranges.remove(idx);
            to = r.to;
        }

        // if let Some(idx) = current_ranges.iter().position(|r| from == r.from && r.direction != range.direction) {
        //     let r = current_ranges.remove(idx);
        //     current_ranges.push(Range { direction: range.direction, from: to, to: r.to });
        // } else if let Some(idx) = current_ranges.iter().position(|r| to == r.from && r.direction != range.direction) {
        //     let r = current_ranges.remove(idx);
        //     current_ranges.push(Range { direction: range.direction, from: r.from, to: from });
        // } else {
        // }

        if let Some(idx) = current_ranges.iter().position(|r|( from == r.from || to == r.to) && r.direction != range.direction) {
            let r = current_ranges.remove(idx);
            if from == r.from && to != r.to {
                if from.abs_diff(to) > r.from.abs_diff(r.to) {
                    current_ranges.push(Range { direction: range.direction, from: r.to, to: to });
                } else {
                    current_ranges.push(Range { direction: r.direction, from: to, to: r.to });
                }
            }
            if from != r.from && to == r.to {
                if from.abs_diff(to) > r.from.abs_diff(r.to) {
                    current_ranges.push(Range { direction: range.direction, from: from, to: r.from });
                } else {
                    current_ranges.push(Range { direction: r.direction, from: r.from, to: from });
                }
            }
        } else {
            current_ranges.push(Range { direction: range.direction, from: from, to: to });
        }



        // println!("CURRENT: {:?} : {current_sum}", current_ranges);
        println!("ANS {ans_1}");
        println!()
    }

    (ans_1 as usize, 0)
}

#[derive(Debug, Clone, Copy)]
struct Range {
    direction: Direction,
    from: i64,
    to: i64,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}