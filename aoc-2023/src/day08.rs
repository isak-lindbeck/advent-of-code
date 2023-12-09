use std::borrow::Cow;
use std::collections::HashMap;

use num::integer::lcm;
use regex_macro::regex;

pub fn run(input: String) -> (usize, usize) {
    let input = clean(&input);
    let step_counter = StepCounter::parse_str(&input);

    let ans_1 = step_counter.count("AAA");
    let ans_2 = step_counter.map.keys()
        .filter(|key| key.ends_with("A"))
        .map(|start| step_counter.count(start))
        .reduce(lcm)
        .unwrap();

    (ans_1, ans_2)
}

fn clean(input: &String) -> Cow<str> {
    regex!("\\(|\\)| =|,").replace_all(&input, "")
}

enum Direction {
    Left,
    Right,
}

struct StepCounter<'a> {
    map: HashMap<&'a str, (&'a str, &'a str)>,
    directions: Vec<Direction>,
}

impl<'a> StepCounter<'_> {
    fn parse_str(input: &str) -> StepCounter {
        let directions = input.lines().next().unwrap().split("")
            .filter(|s| !s.is_empty())
            .map(|s| if s == "L" { Direction::Left } else { Direction::Right })
            .collect();
        let map = input.lines().skip(2).map(|line| {
            let x: Vec<&str> = line.split(" ").collect();
            (x[0], (x[1], x[2]))
        }).collect();

        StepCounter { map, directions }
    }

    fn count(&self, start: &str) -> usize {
        let mut directions = self.directions.iter().cycle();
        let mut current = start;
        let mut count = 0;
        while !current.ends_with("Z") {
            let next_pair = self.map[current];
            current = match directions.next().unwrap() {
                Direction::Left => next_pair.0,
                Direction::Right => next_pair.1,
            };
            count += 1;
        }
        count
    }
}