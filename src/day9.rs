use std::collections::BTreeMap;
use itertools::Itertools;

pub fn solve1(data: Vec<String>) {
    let coords = data
        .iter()
        .map(|s| {
            let s = s
                .split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (s[0], s[1])
        })
        .collect::<Vec<_>>();
    let max_area = coords
        .iter()
        .combinations(2)
        .map(|pair| {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)
        })
        .max()
        .unwrap();
    println!("{}", max_area);
}

fn neighbors(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x > 0 {
        result.push((x - 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if x + 1 < max_x {
        result.push((x + 1, y));
    }
    if y + 1 < max_y {
        result.push((x, y + 1));
    }
    result
}

pub fn solve2(data: Vec<String>) {
    let coords = data
        .iter()
        .map(|s| {
            let s = s
                .split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (s[0], s[1])
        })
        .collect::<Vec<_>>();
    // new -> old
    let x_decompression = coords
        .iter()
        .map(|&(x, _)| x)
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, v)| (i + 1, v)) // leave a border of 0s
        .collect::<BTreeMap<_, _>>();
    let y_decompression = coords
        .iter()
        .map(|&(_, y)| y)
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, v)| (i + 1, v)) // leave a border of 0s
        .collect::<BTreeMap<_, _>>();
    let x_compression = x_decompression
        .iter()
        .map(|(&new, &old)| (old, new))
        .collect::<BTreeMap<_, _>>();
    let y_compression = y_decompression
        .iter()
        .map(|(&new, &old)| (old, new))
        .collect::<BTreeMap<_, _>>();
    let x_max = x_decompression.len();
    let y_max = y_decompression.len();
    // 0 => inside, 1 => edge, 2 => outside
    // Leave a border of 0s around the grid to make flood fill easier
    let mut grid = vec![vec![0; y_max + 2]; x_max + 2];
    for (&(x1, y1), &(x2, y2)) in coords.iter().chain(std::iter::once(&coords[0])).tuple_windows() {
        let cx1 = *x_compression.get(&x1).unwrap();
        let cy1 = *y_compression.get(&y1).unwrap();
        let cx2 = *x_compression.get(&x2).unwrap();
        let cy2 = *y_compression.get(&y2).unwrap();
        if cx1 == cx2 {
            for y in cy1.min(cy2)..=cy1.max(cy2) {
                grid[cx1][y] = 1;
            }
        } else {
            for x in cx1.min(cx2)..=cx1.max(cx2) {
                grid[x][cy1] = 1;
            }
        }
    }
    // Flood fill OUTSIDE because it's hard to find an inside point
    let mut visited = vec![vec![false; y_max + 2]; x_max + 2];
    let mut q = vec![(0, 0)];
    visited[0][0] = true;
    while let Some((x, y)) = q.pop() {
        grid[x][y] = 2;
        for (nx, ny) in neighbors(x, y, grid.len(), grid[0].len()) {
            if !visited[nx][ny] && grid[nx][ny] != 1 {
                visited[nx][ny] = true;
                q.push((nx, ny));
            }
        }
    }
    let max_area = coords
        .iter()
        .combinations(2)
        .filter(|pair| {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            let cx1 = *x_compression.get(&x1).unwrap();
            let cy1 = *y_compression.get(&y1).unwrap();
            let cx2 = *x_compression.get(&x2).unwrap();
            let cy2 = *y_compression.get(&y2).unwrap();
            for x in cx1.min(cx2)..=cx1.max(cx2) {
                if grid[x][cy1] == 2 || grid[x][cy2] == 2 {
                    return false;
                }
            }
            for y in cy1.min(cy2)..=cy1.max(cy2) {
                if grid[cx1][y] == 2 || grid[cx2][y] == 2 {
                    return false;
                }
            }
            true
        })
        .map(|pair| {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)
        })
        .max()
        .unwrap();
    println!("{}", max_area);
}
