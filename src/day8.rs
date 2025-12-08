use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve1(data: Vec<String>) {
    let (num_conn, coords) = data.split_first().unwrap();
    let num_conn = num_conn.parse::<usize>().unwrap();
    let coords = coords
        .iter()
        .map(|s| {
            s.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();
    let mut dist: HashMap<(usize, usize), i64> = HashMap::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let d = (coords[i][0] - coords[j][0]).pow(2)
                + (coords[i][1] - coords[j][1]).pow(2)
                + (coords[i][2] - coords[j][2]).pow(2);
            dist.insert((i, j), d);
        }
    }
    let mut dist = dist.into_iter().collect::<Vec<((usize, usize), i64)>>();
    dist.sort_by_key(|&(_, d)| d);
    let mut adj_list = HashMap::new();
    for &((a, b), _) in dist.iter().take(num_conn) {
        adj_list.entry(a).or_insert(Vec::new()).push(b);
        adj_list.entry(b).or_insert(Vec::new()).push(a);
    }
    let mut visited = HashSet::new();
    let mut components: Vec<u32> = Vec::new();
    for i in 0..coords.len() {
        if visited.contains(&i) {
            continue;
        }
        let mut q = VecDeque::new();
        q.push_back(i);
        visited.insert(i);
        let mut comp_size = 0;
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            comp_size += 1;
            if let Some(neighbors) = adj_list.get(&node) {
                for &neighbor in neighbors.iter() {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        q.push_back(neighbor);
                    }
                }
            }
        }
        components.push(comp_size);
    }
    components.sort();
    components.reverse();
    println!("{}", components.iter().take(3).product::<u32>());
}

pub fn solve2(data: Vec<String>) {
    let (_, coords) = data.split_first().unwrap();
    let coords = coords
        .iter()
        .map(|s| {
            s.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();
    let mut dist: HashMap<(usize, usize), i64> = HashMap::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let d = (coords[i][0] - coords[j][0]).pow(2)
                + (coords[i][1] - coords[j][1]).pow(2)
                + (coords[i][2] - coords[j][2]).pow(2);
            dist.insert((i, j), d);
        }
    }
    let mut dist = dist.into_iter().collect::<Vec<((usize, usize), i64)>>();
    dist.sort_by_key(|&(_, d)| d);
    let mut is_connected = vec![false; coords.len()];
    for &((a, b), _) in dist.iter() {
        is_connected[a] = true;
        is_connected[b] = true;
        if is_connected.iter().all(|&x| x) {
            println!("{}", coords[a][0] * coords[b][0]);
            break;
        }
    }
}
