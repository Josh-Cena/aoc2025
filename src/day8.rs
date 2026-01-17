use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

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
    let mut dist: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let d = (coords[i][0] - coords[j][0]).pow(2)
                + (coords[i][1] - coords[j][1]).pow(2)
                + (coords[i][2] - coords[j][2]).pow(2);
            dist.push((d, i, j));
        }
    }
    let mut heap = BinaryHeap::from_iter(dist.into_iter().map(Reverse));
    let mut uf = QuickUnionUf::<UnionBySize>::new(coords.len());
    for _ in 0..num_conn {
        let Reverse((_, a, b)) = heap.pop().unwrap();
        uf.union(a, b);
    }
    let mut sizes = HashMap::new();
    for i in 0..coords.len() {
        let root = uf.find(i);
        *sizes.entry(root).or_insert(0) += 1;
    }
    let mut v: Vec<usize> = sizes.values().copied().collect();
    v.sort_unstable_by(|a, b| b.cmp(a));
    let result: usize = v.iter().take(3).product();
    println!("{}", result);
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
    let mut dist: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let d = (coords[i][0] - coords[j][0]).pow(2)
                + (coords[i][1] - coords[j][1]).pow(2)
                + (coords[i][2] - coords[j][2]).pow(2);
            dist.push((d, i, j));
        }
    }
    let mut heap = BinaryHeap::from_iter(dist.into_iter().map(Reverse));
    let mut uf = QuickUnionUf::<UnionBySize>::new(coords.len());
    let mut num_components = coords.len();
    loop {
       let Reverse((_, a, b)) = heap.pop().unwrap();
        if uf.union(a, b) {
            num_components -= 1;
        }
        if num_components == 1 {
            println!("{}", coords[a][0] * coords[b][0]);
            break;
        }
    }
}
