use std::cmp::Reverse;
use std::collections::HashMap;
use std::ops::Range;

use priority_queue::PriorityQueue;

pub fn run(input: String) -> (usize, usize) {
    let map = parse_input(input);

    let ans_1 = calculate_distance(&map, (0, 0), 0..3);
    let ans_2 = calculate_distance(&map, (0, 0), 3..10);

    (ans_1, ans_2)
}

fn calculate_distance(map: &Vec<Vec<usize>>, start: (usize, usize), allowed_steps: Range<usize>) -> usize {
    let side = map.len();
    let mut unvisited: PriorityQueue<(usize, usize, bool), Reverse<usize>> = PriorityQueue::new();

    let mut dist: HashMap<(usize, usize, bool), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize, bool), (usize, usize, bool)> = HashMap::new();

    for y in 0..side {
        for x in 0..side {
            if x == 0 && y == 0 {
                continue;
            }
            unvisited.push((x, y, true), Reverse(usize::MAX));
            unvisited.push((x, y, false), Reverse(usize::MAX));
            dist.insert((x, y, true), usize::MAX);
            dist.insert((x, y, false), usize::MAX);
        }
    }
    let (x, y) = start;
    dist.insert((x, y, true), 0);
    dist.insert((x, y, false), 0);
    unvisited.push((x, y, true), Reverse(0));
    unvisited.push((x, y, false), Reverse(0));

    while !unvisited.is_empty() {
        let (node, _) = unvisited.pop().unwrap();
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

        for neighbour in neighbors {
            let (n_x, n_y, _) = neighbour;

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

            let alt = *dist.get(&node).unwrap() + cost;
            if alt < *dist.get(&neighbour).unwrap() {
                dist.insert(neighbour, alt);
                prev.insert(neighbour, node);
                unvisited.change_priority(&neighbour, Reverse(alt));
            }
        }
    }

    let dist_1: &usize = dist.get(&(side - 1, side - 1, true)).unwrap_or(&usize::MAX);
    let dist_2: &usize = dist.get(&(side - 1, side - 1, false)).unwrap_or(&usize::MAX);
    let distance = dist_1.min(dist_2);
    *distance
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
