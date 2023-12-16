use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

const NORTH: u8 = 0b0001;
const SOUTH: u8 = 0b0010;
const WEST: u8 = 0b0100;
const EAST: u8 = 0b1000;

pub fn run(input: String) -> (usize, usize) {
    let side = input.lines().count();
    let map: Vec<Vec<char>> = parse_input(input);

    let ans_1 = calculate(&map, Beam { x: 0, y: 0, direction: EAST });

    let ans_2: usize = (0..side).into_par_iter().map(|i| {
        let north = calculate(&map, Beam { x: i, y: side - 1, direction: NORTH });
        let east = calculate(&map, Beam { x: 0, y: i, direction: EAST });
        let south = calculate(&map, Beam { x: i, y: 0, direction: SOUTH });
        let west = calculate(&map, Beam { x: side - 1, y: i, direction: WEST });
        south.max(north).max(east).max(west)
    }).max().unwrap();

    (ans_1, ans_2)
}

struct Beam {
    x: usize,
    y: usize,
    direction: u8,
}

fn calculate(map: &Vec<Vec<char>>, initial_beam: Beam) -> usize {
    let side = map.len();
    let mut traversed: Vec<Vec<u8>> = vec![vec![0b0000_0000; side]; side];

    let mut beams: Vec<Beam> = Vec::new();
    beams.push(initial_beam);
    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        let x = beam.x;
        let y = beam.y;

        if beam.direction & traversed[x][y] != 0 {
            continue;
        }
        traversed[x][y] = traversed[x][y] | beam.direction;

        let mut next_directions: Vec<u8> = Vec::new();

        match (map[x][y], beam.direction) {
            ('/', NORTH) => next_directions.push(EAST),
            ('/', EAST) => next_directions.push(NORTH),
            ('/', WEST) => next_directions.push(SOUTH),
            ('/', SOUTH) => next_directions.push(WEST),
            ('\\', NORTH) => next_directions.push(WEST),
            ('\\', WEST) => next_directions.push(NORTH),
            ('\\', EAST) => next_directions.push(SOUTH),
            ('\\', SOUTH) => next_directions.push(EAST),
            ('|', EAST | WEST) => {
                next_directions.push(NORTH);
                next_directions.push(SOUTH);
            }
            ('-', NORTH | SOUTH) => {
                next_directions.push(EAST);
                next_directions.push(WEST);
            }
            (_, _) => next_directions.push(beam.direction),
        }

        for dir in next_directions {
            match dir {
                NORTH if y != 0 => beams.push(Beam { x, y: y - 1, direction: NORTH }),
                EAST if x != side - 1 => beams.push(Beam { x: x + 1, y, direction: EAST }),
                SOUTH if y != side - 1 => beams.push(Beam { x, y: y + 1, direction: SOUTH }),
                WEST if x != 0 => beams.push(Beam { x: x - 1, y, direction: WEST }),
                _ => {}
            }
        }
    }

    traversed.iter().flat_map(|col| col.iter()).filter(|traversed| traversed > &&0).count()
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let side = input.lines().count();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; side]; side];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[x][y] = c;
        });
    });
    map
}