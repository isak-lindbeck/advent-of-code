use itertools::Itertools;

pub fn run(input: String) -> (usize, usize) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let seeds = split[0].replace("seeds: ", "").trim().split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>();

    let seed_to_soil: Vec<AlmanacMap> = to_alm_map(split[1]);
    let soil_to_fertilizer: Vec<AlmanacMap> = to_alm_map(split[2]);
    let fertilizer_to_water: Vec<AlmanacMap> = to_alm_map(split[3]);
    let water_to_light: Vec<AlmanacMap> = to_alm_map(split[4]);
    let light_to_temperature: Vec<AlmanacMap> = to_alm_map(split[5]);
    let temperature_to_humidity: Vec<AlmanacMap> = to_alm_map(split[6]);
    let humidity_to_location: Vec<AlmanacMap> = to_alm_map(split[7]);

    let ans_1 = seeds.iter()
        .map(|seed| apply(*seed, &seed_to_soil))
        .map(|seed| apply(seed, &soil_to_fertilizer))
        .map(|seed| apply(seed, &fertilizer_to_water))
        .map(|seed| apply(seed, &water_to_light))
        .map(|seed| apply(seed, &light_to_temperature))
        .map(|seed| apply(seed, &temperature_to_humidity))
        .map(|seed| apply(seed, &humidity_to_location))
        .min().unwrap();

    let ans_2: i64 = seeds.chunks(2)
        .map(|chunk| { Range { start: chunk[0], end: chunk[0] + chunk[1] } })
        .flat_map(|r| apply_range(&r, &seed_to_soil))
        .flat_map(|r| apply_range(&r, &soil_to_fertilizer))
        .flat_map(|r| apply_range(&r, &fertilizer_to_water))
        .flat_map(|r| apply_range(&r, &water_to_light))
        .flat_map(|r| apply_range(&r, &light_to_temperature))
        .flat_map(|r| apply_range(&r, &temperature_to_humidity))
        .flat_map(|r| apply_range(&r, &humidity_to_location))
        .map(|r| r.start)
        .min().unwrap();

    (ans_1 as usize, ans_2 as usize)
}

fn apply(seed: i64, vec: &Vec<AlmanacMap>) -> i64 {
    vec.iter()
        .find(|m| m.start <= seed && seed < m.start + m.range)
        .map(|m| (m.dest_start + seed) - m.start)
        .unwrap_or(seed)
}

fn apply_range(seed_range_in: &Range, maps: &Vec<AlmanacMap>) -> Vec<Range> {
    let mut seed_range = Range { start: seed_range_in.start, end: seed_range_in.end };

    let mut vec = Vec::new();

    for map in maps {
        let delta = map.dest_start - map.start;

        if map.start() > seed_range.end {
            break;
        }
        if seed_range.start > map.end() {
            continue;
        }

        if seed_range.start < map.start() {
            vec.push(Range { start: seed_range.start, end: map.start() - 1 });
            seed_range.start = map.start();
        }

        if seed_range.end > map.end() {
            vec.push(Range { start: seed_range.start + delta, end: map.end() + delta });
            seed_range.start = map.end();
        } else {
            vec.push(Range { start: seed_range.start + delta, end: seed_range.end + delta });
            seed_range = Range { start: 1, end: 0 };
        }
    }

    if seed_range.start < seed_range.end {
        vec.push(seed_range);
    }

    vec
}

fn to_alm_map(x: &str) -> Vec<AlmanacMap> {
    x.split(":\n").last().unwrap().split("\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let maps: Vec<i64> = s.split(" ").map(str::trim).map(|s| s.parse().unwrap()).collect();
            AlmanacMap { dest_start: maps[0], start: maps[1], range: maps[2] }
        })
        .sorted_by(|a, b| a.start.cmp(&b.start))
        .collect()
}

struct AlmanacMap {
    dest_start: i64,
    start: i64,
    range: i64,
}

impl AlmanacMap {
    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn end(&self) -> i64 {
        self.start + self.range
    }
}

struct Range {
    start: i64,
    end: i64,
}