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
            let (label, focal_length) = s.split_once("=").unwrap();
            let focal_length = focal_length.parse().unwrap();
            let hash = hash(label);

            let lens_box: &mut Vec<Lens<'_>> = &mut boxes[hash];
            let box_pos = &lens_box.iter().find_position(|l| l.label == label).map(|(p, _)| p);
            let lens = Lens { label, focal_length };
            if box_pos.is_some() {
                lens_box[box_pos.unwrap()] = lens;
            } else {
                lens_box.push(lens);
            }
        } else {
            let (label, _) = s.split_once("-").unwrap();
            let hash = hash(label);

            let lens_box: &mut Vec<Lens<'_>> = &mut boxes[hash];
            let in_box_pos = &lens_box.iter().find_position(|l| l.label == label).map(|(p, _)| p);
            if in_box_pos.is_some() {
                lens_box.remove(in_box_pos.unwrap());
            }
        }
    }

    let mut ans_2 = 0;

    for (box_idx, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            b.iter().enumerate().for_each(|(slot_idx, l)| {
                ans_2 += (box_idx + 1) * (slot_idx + 1) * l.focal_length
            });
        }
    }

    (ans_1, ans_2)
}

fn hash(s: &str) -> usize {
    s.chars().map(|c| c as usize).fold(0, |hash, ascii| {
        let hash = hash + ascii;
        let hash = hash * 17;
        hash % 256
    })
}

struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}