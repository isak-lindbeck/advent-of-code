use std::cmp::Reverse;
use std::ops::Range;

use priority_queue::PriorityQueue;

pub fn run(input: String) -> (usize, usize) {
    let map = parse_input(input);

    let ans_1 = calculate_djikstra(&map, 0..3);
    let ans_2 = calculate_djikstra(&map, 3..10);

    (ans_1, ans_2)
}

fn calculate_djikstra(map: &Vec<Vec<usize>>, allowed_steps: Range<usize>) -> usize {
    let side = map.len();
    let mut heat_cost: Vec<Vec<Vec<usize>>> = vec![vec![vec![usize::MAX; 2]; side]; side];
    heat_cost[0][0][false as usize] = 0;
    heat_cost[0][0][true as usize] = 0;

    let mut queue: PriorityQueue<(usize, usize, bool), Reverse<usize>> = PriorityQueue::new();
    queue.push((0, 0, true), Reverse(0));
    queue.push((0, 0, false), Reverse(0));

    while let Some((node, _)) = queue.pop() {
        let (x, y, flg) = node;
        if x == side - 1 && y == side - 1 {
            break;
        }
        let neighbors = get_neighbours(&allowed_steps, side, node);
        for neighbour in neighbors {
            let (n_x, n_y, n_flg) = neighbour;
            let move_cost = calculate_cost(map, node, neighbour);

            let new_cost = heat_cost[x][y][flg as usize] + move_cost;
            let prev_cost = heat_cost[n_x][n_y][n_flg as usize];
            if new_cost < prev_cost {
                heat_cost[n_x][n_y][n_flg as usize] = new_cost;
                queue.push(neighbour, Reverse(new_cost));
            }
        }
    }
    let cost_1 = heat_cost[side - 1][side - 1][false as usize];
    let cost_2 = heat_cost[side - 1][side - 1][true as usize];
    cost_1.min(cost_2)
}

fn get_neighbours(allowed_steps: &Range<usize>, side: usize, node: (usize, usize, bool)) -> Vec<(usize, usize, bool)> {
    let (x, y, flg) = node;
    let mut neighbors: Vec<(usize, usize, bool)> = Vec::new();
    if flg {
        allowed_steps.clone().for_each(|dx| {
            if x > dx {
                neighbors.push((x - dx - 1, y, !flg));
            }
            if x < side - dx - 1 {
                neighbors.push((x + dx + 1, y, !flg));
            }
        });
    } else {
        allowed_steps.clone().for_each(|dy| {
            if y > dy {
                neighbors.push((x, y - dy - 1, !flg));
            }
            if y < side - dy - 1 {
                neighbors.push((x, y + dy + 1, !flg));
            }
        });
    }
    neighbors
}

fn calculate_cost(map: &Vec<Vec<usize>>, from: (usize, usize, bool), to: (usize, usize, bool)) -> usize {
    let (f_x, f_y, flg) = from;
    let (t_x, t_y, _) = to;
    let range = match flg {
        true if f_x < t_x => f_x + 1..t_x + 1,
        true => t_x..f_x,
        false if f_y < t_y => f_y + 1..t_y + 1,
        false => t_y..f_y,
    };
    range.map(|i| if flg { map[i][f_y] } else { map[f_x][i]}).sum::<usize>()
}

fn parse_input(input: String) -> Vec<Vec<usize>> {
    let side = input.lines().count();
    let mut map: Vec<Vec<usize>> = vec![vec![0; side]; side];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[x][y] = c.to_digit(10).unwrap() as usize;
        });
    });
    map
}