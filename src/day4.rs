pub fn solve1(data: Vec<String>) {
    let mut total = 0;
    let data = data
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '@' {
                continue;
            }
            let mut neighbors = 0;
            let l = if j > 0 {
                neighbors += if line[j - 1] == '@' { 1 } else { 0 };
                j - 1
            } else {
                0
            };
            let r = if j < line.len() - 1 {
                neighbors += if line[j + 1] == '@' { 1 } else { 0 };
                j + 1
            } else {
                line.len() - 1
            };
            if i > 0 {
                neighbors += data[i - 1][l..=r].iter().filter(|&&x| x == '@').count();
            }
            if i < data.len() - 1 {
                neighbors += data[i + 1][l..=r].iter().filter(|&&x| x == '@').count();
            }
            if neighbors < 4 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

pub fn solve2(data: Vec<String>) {
    let mut data = data
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut total = 0;
    loop {
        let mut positions: Vec<(usize, usize)> = vec![];
        for (i, line) in data.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c != '@' {
                    continue;
                }
                let mut neighbors = 0;
                let l = if j > 0 {
                    neighbors += if line[j - 1] == '@' { 1 } else { 0 };
                    j - 1
                } else {
                    0
                };
                let r = if j < line.len() - 1 {
                    neighbors += if line[j + 1] == '@' { 1 } else { 0 };
                    j + 1
                } else {
                    line.len() - 1
                };
                if i > 0 {
                    neighbors += data[i - 1][l..=r].iter().filter(|&&x| x == '@').count();
                }
                if i < data.len() - 1 {
                    neighbors += data[i + 1][l..=r].iter().filter(|&&x| x == '@').count();
                }
                if neighbors < 4 {
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
