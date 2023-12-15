use itertools::{Itertools};

pub fn run(input: String) -> (usize, usize) {
    let strings: Vec<_> = input.split(",").collect();
    let mut ans_1 = 0;
    for s in &strings {
        let hash = hash(s);
        ans_1 = ans_1 + hash;
    }

    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    (0..256).for_each(|_| boxes.push(Vec::new()));

    for s in &strings {
        if s.contains("=") {
            let (label, fl) = s.split_once("=").unwrap();
            let hash = hash(label);

            let lens_box: &mut Vec<Lens<'_>> = &mut boxes[hash];

            let in_box_pos = &lens_box.iter().find_position(|l| l.label == label);
            let in_box_pos = in_box_pos.map(|(pos, _)| pos);
            let fl = fl.parse().unwrap();

            let lens = Lens { label, fl };
            if in_box_pos.is_some() {
                lens_box[in_box_pos.unwrap()] = lens;
            } else {
                lens_box.push(lens);
            }
        } else {
            let (label, _) = s.split_once("-").unwrap();
            let hash = hash(label);

            let lens_box: &mut Vec<Lens<'_>> = &mut boxes[hash];
            let in_box_pos = &lens_box.iter().find_position(|l| l.label == label);
            let in_box_pos = in_box_pos.map(|(pos, _)| pos);

            if in_box_pos.is_some() {
                lens_box.remove(in_box_pos.unwrap());
            }
        }
    }

    let mut ans_2 = 0;

    for (i, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            b.iter().enumerate().for_each(|(s, l)| {
                ans_2 += (i + 1) * (s + 1) * l.fl

            });
        }
    }

    (ans_1, ans_2)
}

fn hash(s: &str) -> usize {
    let mut hash = 0;
    s.chars().map(|c| c as usize).for_each(|ascii| {
        hash = hash + ascii;
        hash = hash * 17;
        hash = hash % 256;
    });
    hash
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    fl: usize,
}