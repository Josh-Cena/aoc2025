use itertools::Itertools;
use plotters::prelude::*;
use std::collections::BTreeMap;

fn draw_polyline(points: &[(i64, i64)], name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (mut min_x, mut max_x) = (i64::MAX, i64::MIN);
    let (mut min_y, mut max_y) = (i64::MAX, i64::MIN);

    for &(x, y) in points {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    min_x -= (max_x - min_x).abs() / 20;
    max_x += (max_x - min_x).abs() / 20;
    min_y -= (max_y - min_y).abs() / 20;
    max_y += (max_y - min_y).abs() / 20;

    // Small padding so the line isn't clipped
    let pad = 1;
    let x_range = (min_x - pad)..(max_x + pad);
    let y_range = (min_y - pad)..(max_y + pad);

    let root = BitMapBackend::new(name, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(0)
        .x_label_area_size(0)
        .y_label_area_size(0)
        .build_cartesian_2d(x_range, y_range)?;

    chart.draw_series(std::iter::once(PathElement::new(points, &BLACK)))?;

    root.present()?;
    Ok(())
}

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
    draw_polyline(&coords, "src/day9.png").unwrap();
    // new -> old
    let x_decompressor = coords
        .iter()
        .map(|&(x, _)| x)
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, v)| (i + 1, v)) // leave a border of 0s
        .collect::<BTreeMap<_, _>>();
    let y_decompressor = coords
        .iter()
        .map(|&(_, y)| y)
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, v)| (i + 1, v)) // leave a border of 0s
        .collect::<BTreeMap<_, _>>();
    let x_compressor = x_decompressor
        .iter()
        .map(|(&new, &old)| (old, new))
        .collect::<BTreeMap<_, _>>();
    let y_compressor = y_decompressor
        .iter()
        .map(|(&new, &old)| (old, new))
        .collect::<BTreeMap<_, _>>();
    draw_polyline(
        coords
            .iter()
            .map(|&(x, y)| (x_compressor[&x] as i64, y_compressor[&y] as i64))
            .collect::<Vec<_>>()
            .as_slice(),
        "src/day9_compressed.png",
    )
    .unwrap();
    let x_max = x_decompressor.len();
    let y_max = y_decompressor.len();
    // 0 => inside, 1 => edge, 2 => outside
    // Leave a border of 0s around the grid to make flood fill easier
    let mut grid = vec![vec![0; y_max + 2]; x_max + 2];
    for (&(x1, y1), &(x2, y2)) in coords
        .iter()
        .chain(std::iter::once(&coords[0]))
        .tuple_windows()
    {
        let cx1 = *x_compressor.get(&x1).unwrap();
        let cy1 = *y_compressor.get(&y1).unwrap();
        let cx2 = *x_compressor.get(&x2).unwrap();
        let cy2 = *y_compressor.get(&y2).unwrap();
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
    let mut st = vec![(0, 0)];
    visited[0][0] = true;
    while let Some((x, y)) = st.pop() {
        grid[x][y] = 2;
        for (nx, ny) in neighbors(x, y, grid.len(), grid[0].len()) {
            if !visited[nx][ny] && grid[nx][ny] != 1 {
                visited[nx][ny] = true;
                st.push((nx, ny));
            }
        }
    }
    let max_area = coords
        .iter()
        .combinations(2)
        .filter(|pair| {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            let cx1 = *x_compressor.get(&x1).unwrap();
            let cy1 = *y_compressor.get(&y1).unwrap();
            let cx2 = *x_compressor.get(&x2).unwrap();
            let cy2 = *y_compressor.get(&y2).unwrap();
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
