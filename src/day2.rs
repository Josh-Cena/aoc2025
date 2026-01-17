fn ceil_div(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

pub fn solve1(data: Vec<String>) {
    let ranges: Vec<(u64, u64)> = data[0]
        .split(',')
        .map(|s| {
            let parts: Vec<&str> = s.split('-').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();
    let mut total = 0;
    for (low, high) in &ranges {
        // Solve inequality: a * (10^d + 1) >= l, where d is the number of
        // digits in a. For each d, a is in the range [10^(d-1), 10^d - 1],
        // So the smallest d is (10^d - 1) * (10^d + 1) >= l.
        let d_min = ((*low as f64 + 1.0).log10() / 2.0).ceil() as u32;
        // Solve inequality: a * (10^d + 1) <= r,
        // which is (10^(d-1) * (10^d + 1)) <= r.
        let d_max = (((40 * *high + 1) as f64).sqrt() / 2.0 - 0.5)
            .log10()
            .floor() as u32;
        for d in d_min..=d_max {
            let multiplier = 10u64.pow(d) + 1;
            let a_min = ceil_div(*low, multiplier).max(10u64.pow(d - 1));
            let a_max = (*high / multiplier).min(10u64.pow(d) - 1);
            if a_min > a_max {
                continue;
            }
            total += (a_max - a_min + 1) * (a_max + a_min) * multiplier / 2;
        }
    }
    println!("{}", total);
}

pub fn solve2(data: Vec<String>) {
    let ranges: Vec<(u64, u64)> = data[0]
        .split(',')
        .map(|s| {
            let parts: Vec<&str> = s.split('-').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();
    let candidates = vec![
        (11, 1, 9),
        (111, 1, 9),
        (11111, 1, 9),
        (1111111, 1, 9),
        (101, 10, 99),
        (10101, 10, 99),
        (1010101, 10, 99),
        (101010101, 10, 99),
        (1001, 100, 999),
        (1001001, 100, 999),
        (1001001001, 100, 999),
        (10001, 1000, 9999),
        (100001, 10000, 99999),
        (1001001, 100, 999),
    ];
    let mut total = 0;
    // TODO: do better than enumerating all numbers in range
    // But it needs to work with double-counting, for example `222222` can be
    // counted as `22 * 10101`, or `222 * 1001`
    for (low, high) in &ranges {
        for num in *low..=*high {
            for (multiplier, min, max) in &candidates {
                if num % multiplier == 0 {
                    let n = num / multiplier;
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
