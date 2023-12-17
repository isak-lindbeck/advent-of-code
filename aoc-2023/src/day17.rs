use std::cmp::Reverse;
use std::ops::Range;

use priority_queue::PriorityQueue;

pub fn run(input: String) -> (usize, usize) {
    let map = parse_input(input);

    let ans_1 = calculate_distance(&map, 0..3);
    let ans_2 = calculate_distance(&map, 3..10);

    (ans_1, ans_2)
}

fn calculate_distance(map: &Vec<Vec<usize>>, allowed_steps: Range<usize>) -> usize {
    let side = map.len();
    let mut dist: Vec<Vec<Vec<usize>>> = vec![vec![vec![usize::MAX; 2]; side]; side];
    dist[0][0][false as usize] = 0;
    dist[0][0][true as usize] = 0;

    let mut queue: PriorityQueue<(usize, usize, bool), Reverse<usize>> = PriorityQueue::new();
    queue.push((0, 0, true), Reverse(0));
    queue.push((0, 0, false), Reverse(0));

    while !queue.is_empty() {
        let (node, _) = queue.pop().unwrap();
        let (x, y, flg) = node;

        if x == side - 1 && y == side - 1 {
            break;
        }

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
        for neighbour in neighbors {
            let (n_x, n_y, n_flg) = neighbour;

            let cost = if flg {
                if x < n_x {
                    (x + 1..=n_x).map(|i| map[i][y]).sum::<usize>()
                } else {
                    (n_x..x).map(|i| map[i][y]).sum::<usize>()
                }
            } else {
                if y < n_y {
                    (y + 1..=n_y).map(|i| map[x][i]).sum::<usize>()
                } else {
                    (n_y..y).map(|i| map[x][i]).sum::<usize>()
                }
            };

            let alt = dist[x][y][flg as usize] + cost;
            if alt < dist[n_x][n_y][n_flg as usize] {
                dist[n_x][n_y][n_flg as usize] = alt;
                queue.push(neighbour, Reverse(alt));
            }
        }
    }

    let dist_1 = dist[side - 1][side - 1][false as usize];
    let dist_2 = dist[side - 1][side - 1][true as usize];
    dist_1.min(dist_2)
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