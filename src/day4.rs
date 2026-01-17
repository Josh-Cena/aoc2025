fn neighbors(data: &Vec<Vec<char>>, i: isize, j: isize, w: isize, h: isize) -> usize {
    let mut count = 0;
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            let ni = i + di;
            let nj = j + dj;
            if ni >= 0 && ni < h && nj >= 0 && nj < w && data[ni as usize][nj as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

pub fn solve1(data: Vec<String>) {
    let mut total = 0;
    let data = data
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let w = data[0].len() as isize;
    let h = data.len() as isize;
    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '@' {
                continue;
            }
            if neighbors(&data, i as isize, j as isize, w, h) < 4 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

pub fn solve2(data: Vec<String>) {
    let mut total = 0;
    let mut data = data
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let w = data[0].len() as isize;
    let h = data.len() as isize;
    loop {
        let mut positions: Vec<(usize, usize)> = vec![];
        for (i, line) in data.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c != '@' {
                    continue;
                }
                if neighbors(&data, i as isize, j as isize, w, h) < 4 {
                    positions.push((i, j));
                }
            }
        }
        if positions.is_empty() {
            break;
        }
        for (i, j) in positions.iter() {
            data[*i][*j] = '.';
        }
        total += positions.len();
    }
    println!("{}", total);
}
