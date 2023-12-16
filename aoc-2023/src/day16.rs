use std::cmp;

const NORTH: u8 = 0b0000_0001;
const SOUTH: u8 = 0b0000_0010;
const WEST: u8 = 0b0000_0100;
const EAST: u8 = 0b0000_1000;

pub fn run(input: String) -> (usize, usize) {
    let side = input.lines().count();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; side]; side];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[x][y] = c;
        });
    });

    let initial_beam = Beam { x: 0, y: 0, direction: EAST };
    let ans_1 = calculate(&map, initial_beam);

    let mut ans_2 = 0;
    for i in 0..side {
        let initial_beam = Beam { x: i, y: 0, direction: SOUTH };
        ans_2 = cmp::max(ans_2, calculate(&map, initial_beam));
        let initial_beam = Beam { x: i, y: side - 1, direction: NORTH };
        ans_2 = cmp::max(ans_2, calculate(&map, initial_beam));
        let initial_beam = Beam { x: 0, y: i, direction: EAST };
        ans_2 = cmp::max(ans_2, calculate(&map, initial_beam));
        let initial_beam = Beam { x: side - 1, y: i, direction: WEST };
        ans_2 = cmp::max(ans_2, calculate(&map, initial_beam));
    }

    (ans_1, ans_2)
}

fn calculate(map: &Vec<Vec<char>>, initial_beam: Beam) -> usize {
    let side = map.len();
    let mut passed: Vec<Vec<u8>> = vec![vec![0b0000_0000; side]; side];

    let mut beams: Vec<Beam> = Vec::new();
    beams.push(initial_beam);

    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        let mut x = beam.x;
        let mut y = beam.y;

        let tile = passed[x][y];
        if beam.direction & tile != 0 {
            continue;
        }
        passed[x][y] = tile | beam.direction;

        let mut next_directions: Vec<u8> = Vec::new();
        match map[x][y] {
            '/' => {
                match beam.direction {
                    NORTH => next_directions.push(EAST),
                    EAST => next_directions.push(NORTH),
                    WEST => next_directions.push(SOUTH),
                    SOUTH => next_directions.push(WEST),
                    _ => panic!("Unknown direction")
                }
            }
            '\\' => {
                match beam.direction {
                    NORTH => next_directions.push(WEST),
                    WEST => next_directions.push(NORTH),
                    EAST => next_directions.push(SOUTH),
                    SOUTH => next_directions.push(EAST),
                    _ => panic!("Unknown direction")
                }
            }
            '|' => {
                match beam.direction {
                    EAST | WEST => {
                        next_directions.push(NORTH);
                        next_directions.push(SOUTH);
                    }
                    _ => next_directions.push(beam.direction)
                }
            }
            '-' => {
                match beam.direction {
                    NORTH | SOUTH => {
                        next_directions.push(EAST);
                        next_directions.push(WEST);
                    }
                    _ => next_directions.push(beam.direction)
                }
            }
            '.' => next_directions.push(beam.direction),
            _ => panic!("Unknown char {}", map[x][y]),
        }

        // Move
        for dir in next_directions {
            match dir {
                NORTH if y != 0 => beams.push(Beam { x: x, y: y - 1, direction: NORTH }),
                EAST if x != side - 1 => beams.push(Beam { x: x + 1, y: y, direction: EAST }),
                WEST if x != 0 => beams.push(Beam { x: x - 1, y: y, direction: WEST }),
                SOUTH if y != side - 1 => beams.push(Beam { x: x, y: y + 1, direction: SOUTH }),
                _ => {},
            }
        }
    }

    let mut energized = 0;
    for y in 0..side {
        for x in 0..side {
            let p = passed[x][y];
            if p > 0 {
                energized += 1;
            }
        }
    }
    energized
}

struct Beam {
    x: usize,
    y: usize,
    direction: u8,
}