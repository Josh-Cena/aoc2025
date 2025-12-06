pub fn solve1(data: Vec<String>) {
    let mut ranges = vec![];
    let mut total = 0;
    for line in data {
        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            ranges.push((start, end));
        } else {
            let num: u64 = line.parse().unwrap();
            if ranges.iter().any(|&(s, e)| num >= s && num <= e) {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

pub fn solve2(data: Vec<String>) {
    let mut ranges = vec![];
    for line in data {
        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            ranges.push((start, end));
        } else {
            break;
        }
    }
    ranges.sort_by_key(|k| k.0);
    let mut last_end = ranges[0].1;
    let mut total = last_end - ranges[0].0 + 1;
    for (start, end) in ranges.iter().skip(1) {
        if *start > last_end {
            total += end - start + 1;
            last_end = *end;
        } else if *end > last_end {
            total += end - last_end;
            last_end = *end;
        }
    }
    println!("{}", total);
}
