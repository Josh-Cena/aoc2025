pub fn solve1(data: Vec<String>) {
    let ranges: Vec<(u64, u64)> = data[0].split(',').map(|s| {
        let parts: Vec<&str> = s.split('-').collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }).collect();
    let candidates = vec![11, 101, 1001, 10001, 100001];
    let mut total = 0;
    for candidate in candidates {
        let min = (candidate - 1) / 10;
        let max = candidate - 2;
        for (low, high) in &ranges {
            let n_low = u64::div_ceil(*low, candidate);
            let n_high = high / candidate;
            let n_low = std::cmp::max(n_low, min);
            let n_high = std::cmp::min(n_high, max);
            if n_high >= n_low {
                total += (n_low + n_high) * (n_high - n_low + 1) / 2 * candidate;
            }
        }
    }
    println!("{}", total);
}

pub fn solve2(data: Vec<String>) {
    let ranges: Vec<(u64, u64)> = data[0].split(',').map(|s| {
        let parts: Vec<&str> = s.split('-').collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }).collect();
    let candidates = vec![
        (11, 1, 9), (111, 1, 9), (11111, 1, 9), (1111111, 1, 9),
        (101, 10, 99), (10101, 10, 99), (1010101, 10, 99), (101010101, 10, 99),
        (1001, 100, 999), (1001001, 100, 999), (1001001001, 100, 999),
        (10001, 1000, 9999),
        (100001, 10000, 99999),
        (1001001, 100, 999),
    ];
    let mut total = 0;
    for (low, high) in &ranges {
        for num in *low..=*high {
            for (candidate, min, max) in &candidates {
                if num % candidate == 0 {
                    let n = num / candidate;
                    if n >= *min && n <= *max {
                        total += num;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", total);
}
