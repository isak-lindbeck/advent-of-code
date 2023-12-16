const NORTH: u8 = 0b0000_0001;
const SOUTH: u8 = 0b0000_0010;
const WEST: u8 = 0b0000_0100;
const EAST: u8 = 0b0000_1000;

pub fn run(input: String) -> (usize, usize) {
    let side = input.lines().count();
    let mut passed: Vec<Vec<u8>> = vec![vec![0b0000_0000; side]; side];
    let mut map: Vec<Vec<char>> = vec![vec!['.'; side]; side];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[x][y] = c;
        });
    });

    let mut beams: Vec<Beam> = Vec::new();
    beams.push(Beam { x: 0, y: 0, direction: EAST });

    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        let mut x = beam.x;
        let mut y = beam.y;

        let tile = passed[x][y];
        if beam.direction & tile != 0 {
            // Already passed
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

    let mut ans_1 = 0;
    for y in 0..side {
        for x in 0..side {
            let p = passed[x][y];
            if p > 0 {
                match p {
                    NORTH => print!("^"),
                    SOUTH => print!("v"),
                    EAST => print!(">"),
                    WEST => print!("<"),
                    _ => print!("#"),
                }

                ans_1 += 1;
            } else {
                print!(".")
            }
        }
        println!()
    }

    (ans_1, 0)
}

struct Beam {
    x: usize,
    y: usize,
    direction: u8,
}