use itertools::Itertools;

pub fn run(input: String) -> (usize, usize) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let seeds = split[0].replace("seeds: ", "").split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut maps: Vec<AlmanacMap> = Vec::new();
    for i in 1..=7 {
        maps.push(AlmanacMap { guides: to_alm_map(split[i]) })
    }

    let ans_1 = seeds.iter().map(|s| { maps.iter().fold(*s, |s, m| m.apply(s)) }).min().unwrap();

    let ans_2 = seeds.chunks(2)
        .map(|chunk| { Range { start: chunk[0], end: chunk[0] + chunk[1] } })
        .map(|range| maps.iter()
            .fold(vec!(range), |ranges, m| m.apply_guides(ranges))
            .iter().map(|r| r.start).min().unwrap()).min().unwrap();

    (ans_1 as usize, ans_2 as usize)
}

fn to_alm_map(x: &str) -> Vec<RangeGuide> {
    x.split(":\n").last().unwrap().split("\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let maps: Vec<i64> = s.split(" ").map(str::trim).map(|s| s.parse().unwrap()).collect();
            RangeGuide { dest_start: maps[0], range: Range { start: maps[1], end: maps[1] + maps[2] } }
        })
        .sorted_by(|a, b| a.range.start.cmp(&b.range.start))
        .collect()
}

struct Range {
    start: i64,
    end: i64,
}

struct RangeGuide {
    dest_start: i64,
    range: Range,
}

struct AlmanacMap {
    guides: Vec<RangeGuide>,
}

impl AlmanacMap {
    fn apply(&self, seed: i64) -> i64 {
        self.guides.iter()
            .find(|m| m.range.start <= seed && seed < (m.range.end))
            .map(|m| (m.dest_start + seed) - m.range.start)
            .unwrap_or(seed)
    }

    fn apply_guides(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut v: Vec<Range> = Vec::new();
        for range in ranges {
            self.split_range(&range, &mut v);
        }
        v
    }

    fn split_range(&self, seed_range_in: &Range, vec: &mut Vec<Range>) {
        let mut seed_range = Range { start: seed_range_in.start, end: seed_range_in.end };
        for map in &self.guides {
            if map.range.start > seed_range.end {
                break;
            }
            if seed_range.start > map.range.end {
                continue;
            }

            if seed_range.start < map.range.start {
                vec.push(Range { start: seed_range.start, end: map.range.start - 1 });
                seed_range.start = map.range.start;
            }

            let delta = map.dest_start - map.range.start;
            if seed_range.end > map.range.end {
                vec.push(Range { start: seed_range.start + delta, end: map.range.end + delta });
                seed_range.start = map.range.end;
            } else {
                vec.push(Range { start: seed_range.start + delta, end: seed_range.end + delta });
                seed_range = Range { start: 1, end: 0 };
            }
        }

        if seed_range.start < seed_range.end {
            vec.push(seed_range);
        }
    }
}