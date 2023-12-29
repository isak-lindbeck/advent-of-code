use std::collections::HashMap;
use priority_queue::PriorityQueue;

pub fn run(input: String) -> (usize, usize) {
    let side = input.lines().count();
    let mut map: Vec<Vec<char>> = vec![vec!['#'; side]; side];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[x][y] = c;
        });
    });

    let start = (1, 0);
    let mut graph: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut nodes: HashMap<(usize, usize), usize> = HashMap::new();

    let start_idx = 0;
    nodes.insert(start, start_idx);
    graph.push(Vec::new());
    let end_idx = 1;
    nodes.insert((side - 2, side - 1), end_idx);
    graph.push(Vec::new());

    for y in 1..side - 1 {
        for x in 1..side - 1 {
            let v = [map[x - 1][y], map[x + 1][y], map[x][y - 1], map[x][y + 1]];
            if (map[x - 1][y] == '>' || map[x][y - 1] == 'v') && (map[x + 1][y] == '>' || map[x][y + 1] == 'v') {
                let node_idx = nodes.len();
                nodes.insert((x, y), node_idx);
                graph.push(Vec::new());
            }
        }
    }

    nodes.iter().for_each(|(&(x, y), &node_idx)| {
        let neighbours = get_node_neighbours(&map, (x, y));
        neighbours.iter().for_each(|&n| {
            let mut prev = (x, y);
            let mut current = n;
            let mut count = 1;
            while !nodes.contains_key(&current) {
                let next = walk(&map, current, prev);
                prev = current;
                current = next;
                count += 1;
            }
            let c_idx = nodes.get(&current).unwrap();
            graph[node_idx].push((*c_idx, count));
            // println!("{x},{y} -> {},{}: ({count})", current.0, current.1);
        });
    });

    // println!("nodes: {:?}", nodes);
    // println!();
    // println!("{:?}", graph);
    // println!();

    let ans_1 = calculate_djikstra(&graph);


    (ans_1, 0)
}

fn get_node_neighbours(map: &Vec<Vec<char>>, current: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = current;
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if x > 0 {
        if map[x - 1][y] != '#' {
            neighbors.push((x - 1, y));
        }
    }
    if x < map.len() - 1 {
        if map[x + 1][y] != '#' {
            neighbors.push((x + 1, y));
        }
    }
    if y > 0 {
        if map[x][y - 1] != '#' {
            neighbors.push((x, y - 1));
        }
    }
    if y < map.len() - 1 {
        if map[x][y + 1] != '#' {
            neighbors.push((x, y + 1));
        }
    }
    neighbors
}

fn get_node_neighbours_slopes(map: &Vec<Vec<char>>, current: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = current;
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if x < map.len() - 1 {
        if map[x + 1][y] != '#' {
            neighbors.push((x + 1, y));
        }
    }
    if y < map.len() - 1 {
        if map[x][y + 1] != '#' {
            neighbors.push((x, y + 1));
        }
    }
    neighbors
}

fn walk(map: &Vec<Vec<char>>, current: (usize, usize), prev: (usize, usize)) -> (usize, usize) {
    let (x, y) = current;
    if !(prev == (x - 1, y)) && map[x - 1][y] != '#' {
        return (x - 1, y);
    } else if !(prev == (x + 1, y)) && map[x + 1][y] != '#' {
        return (x + 1, y);
    } else if !(prev == (x, y - 1)) && map[x][y - 1] != '#' {
        return (x, y - 1);
    } else {
        return (x, y + 1);
    }
}

fn calculate_djikstra(graph: &Vec<Vec<(usize, usize)>>) -> usize {
    let side = graph.len();
    let mut distance: Vec<usize> = vec![0; side];

    let mut queue: PriorityQueue<(usize, Vec<usize>), usize> = PriorityQueue::new();
    queue.push((0, Vec::new()), 0);

    while let Some(((node_idx, visited), prev_cost)) = queue.pop() {
        for &(neighbour_idx, cost) in graph[node_idx].iter() {
            if visited.contains(&neighbour_idx) {
                continue;
            }
            let new_cost = prev_cost + cost;
            if new_cost > distance[neighbour_idx] {
                distance[neighbour_idx] = new_cost;
            }
            let mut vis = visited.clone();
            vis.push(neighbour_idx);
            queue.push((neighbour_idx, vis), new_cost);
        }
    }
    let end_cost = distance[1];
    end_cost
}