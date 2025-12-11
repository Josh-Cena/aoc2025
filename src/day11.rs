use core::num;
use std::collections::HashMap;
use std::collections::VecDeque;

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    avoid: &[&str],
) -> i64 {
    let mut num_paths: HashMap<String, i64> = HashMap::new();
    let mut indeg: HashMap<String, i32> = HashMap::new();
    for (u, neighbors) in graph {
        if avoid.contains(&u.as_str()) {
            continue;
        }
        indeg.entry(u.clone()).or_insert(0);
        num_paths.insert(u.clone(), 0);
        for v in neighbors {
            if avoid.contains(&v.as_str()) {
                continue;
            }
            *indeg.entry(v.clone()).or_insert(0) += 1;
        }
    }

    *num_paths.get_mut(start).unwrap() = 1;

    let mut queue = VecDeque::from_iter(
        indeg
            .iter()
            .filter_map(|(v, &d)| if d == 0 { Some(v.clone()) } else { None }),
    );

    while let Some(u) = queue.pop_front() {
        let count_u = num_paths[&u];
        if let Some(neighbors) = graph.get(&u) {
            for v in neighbors {
                if avoid.contains(&v.as_str()) {
                    continue;
                }
                if let Some(d) = indeg.get_mut(v) {
                    *num_paths.entry(v.clone()).or_insert(0) += count_u;
                    *d -= 1;
                    if *d == 0 {
                        queue.push_back(v.clone());
                    }
                }
            }
        }
    }

    *num_paths.get(end).unwrap_or(&0)
}

pub fn solve1(data: Vec<String>) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in data {
        let parts = line.split(": ").collect::<Vec<_>>();
        let node = parts[0].to_string();
        let neighbors = parts[1]
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        graph.insert(node, neighbors);
    }
    let num_paths = count_paths(&graph, "you", "out", &[]);
    println!("{}", num_paths);
}

pub fn solve2(data: Vec<String>) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in data {
        let parts = line.split(": ").collect::<Vec<_>>();
        let node = parts[0].to_string();
        let neighbors = parts[1]
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        graph.insert(node, neighbors);
    }
    let total = count_paths(&graph, "svr", "out", &[])
        - count_paths(&graph, "svr", "out", &["dac"])
        - count_paths(&graph, "svr", "out", &["fft"])
        + count_paths(&graph, "svr", "out", &["dac", "fft"]);
    println!("{}", total);
}
