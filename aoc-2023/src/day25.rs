use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn run(input: String) -> (usize, usize) {
    let mut graph: Vec<Vec<usize>> = Vec::new();
    let mut node_ids: HashMap<&str, usize> = HashMap::new();
    input.lines()
        .for_each(|l| {
            let (a, b) = l.split_once(": ").unwrap();
            let connected: Vec<&str> = b.split(" ").collect();
            if !node_ids.keys().contains(&a) {
                let nxt_idx = node_ids.len();
                node_ids.insert(&a, nxt_idx);
                graph.push(Vec::new());
            }
            let a = node_ids[a];

            connected.iter().for_each(|c| {
                if !node_ids.keys().contains(&c) {
                    let nxt_idx = node_ids.len();
                    node_ids.insert(&c, nxt_idx);
                    graph.push(Vec::new());
                }
                let c = node_ids[c];

                graph[a].push(c);
                graph[c].push(a);
            })
        });

    let start = 0;
    let mut same_cluster: HashSet<usize> = HashSet::new();
    same_cluster.insert(start);
    for idx in 0..graph.len() {
        let mut traversed: HashSet<(usize, usize)> = HashSet::new();
        (0..3).for_each(|_| {
            if let Some(path) = djikstra(&graph, start, idx, &mut traversed) {
                path.windows(2).for_each(|i| {
                    traversed.insert((i[1], i[0]));
                });
            }
        });

        let path = djikstra(&graph, start, idx, &mut traversed);
        if path.is_some() {
            same_cluster.insert(idx);
        }
    }
    let ans_1 = same_cluster.len() * (graph.len() - same_cluster.len());
    (ans_1, 0)
}

fn djikstra(graph: &Vec<Vec<usize>>, from: usize, to: usize, traversed: &mut HashSet<(usize, usize)>) -> Option<Vec<usize>> {
    let mut dist = vec![usize::MAX; graph.len()];
    let mut prev = vec![None; graph.len()];
    dist[from] = 0;

    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(from);
    while let Some(cur) = queue.pop_front() {
        if cur == to {
            break;
        }
        for &n in &graph[cur] {
            if traversed.contains(&(cur, n)) {
                continue;
            }
            let alt = dist[cur] + 1;
            if alt < dist[n] {
                dist[n] = alt;
                prev[n] = Some(cur);
                queue.push_back(n);
            }
        }
    }

    if prev[to].is_none() {
        return None;
    }

    let mut path: Vec<usize> = Vec::new();
    let mut current = to;
    path.push(current);
    while let Some(p) = prev[current] {
        current = p;
        path.push(current);
    }
    Some(path)
}