use priority_queue::PriorityQueue;

pub fn run(input: String) -> (usize, usize) {
    let side = input.lines().count();
    let mut map: Vec<Vec<char>> = vec![vec!['#'; side]; side];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[x][y] = c;
        });
    });

    let x = calculate_djikstra(&map);

    (x, 0)
}


fn calculate_djikstra(map: &Vec<Vec<char>>) -> usize {
    let side = map.len();
    let mut steps: Vec<Vec<usize>> = vec![vec![0; side]; side];
    steps[1][0] = 0;
    let mut queue: PriorityQueue<(usize, usize, Vec<(usize, usize)>), usize> = PriorityQueue::new();
    queue.push((1, 0, vec![(1,0)]), 0);

    while let Some((node, _)) = queue.pop() {
        let (x, y, visited) = node;
        // if x == side - 1 && y == side - 1 {
        //     break;
        // }
        let neighbors = get_neighbours(&map, (x, y, visited.clone()));
        for neighbour in neighbors {
            let (n_x, n_y) = neighbour;
            let new_steps = steps[x][y] + 1;
            let prev_steps = steps[n_x][n_y];
            if new_steps > prev_steps {
                steps[n_x][n_y] = new_steps;
                let mut vis = visited.clone();
                vis.push((n_x, n_y));
                queue.push((n_x, n_y, vis), new_steps);
            }
        }
    }
    let steps = steps[side - 2][side - 1];
    steps
}

fn get_neighbours(map: &Vec<Vec<char>>, node: (usize, usize, Vec<(usize, usize)>)) -> Vec<(usize, usize)> {
    let (x, y, visited) = node;
    let c = map[x][y];
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if x > 0 && (c == '.' || c == '<') {
        if !visited.contains(&(x - 1, y)) && map[x - 1][y] != '#' {
            neighbors.push((x - 1, y));
        }
    }
    if x < map.len() - 1 && (c == '.' || c == '>') {
        if !visited.contains(&(x + 1, y)) && map[x + 1][y] != '#' {
            neighbors.push((x + 1, y));
        }
    }
    if y > 0 && (c == '.' || c == '^') {
        if !visited.contains(&(x, y - 1)) && map[x][y - 1] != '#' {
            neighbors.push((x, y - 1));
        }
    }
    if y < map.len() - 1 && (c == '.' || c == 'v') {
        if !visited.contains(&(x, y + 1)) && map[x][y + 1] != '#' {
            neighbors.push((x, y + 1));
        }
    }
    neighbors
}