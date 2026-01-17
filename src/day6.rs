pub fn solve1(data: Vec<String>) {
    let (operators, numbers) = data.split_last().unwrap();
    let operators = operators.split_whitespace().collect::<Vec<&str>>();
    let numbers = numbers
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();
    let total = operators
        .iter()
        .enumerate()
        .map(|(i, &op)| match op {
            "+" => numbers.iter().map(|nums| nums[i]).sum(),
            "*" => numbers.iter().map(|nums| nums[i]).product(),
            _ => 0,
        })
        .sum::<i64>();
    println!("{}", total);
}

pub fn solve2(data: Vec<String>) {
    let mut transposed: Vec<Vec<char>> = vec![Vec::new(); data[0].chars().count()];
    for line in data.iter() {
        for (i, c) in line.chars().enumerate() {
            transposed[i].push(c);
        }
    }
    let mut operators = Vec::new();
    let mut numbers = Vec::new();
    for col in transposed.iter_mut() {
        let last = *col.last().unwrap();
        if matches!(last, '+' | '*') {
            operators.push(last);
            col.pop();

            numbers.push(Vec::new());
        }

        let line = col.iter().collect::<String>();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let val: i64 = col.iter().collect::<String>().trim().parse().unwrap();
        numbers.last_mut().unwrap().push(val);
    }
    let total = operators
        .iter()
        .enumerate()
        .map(|(i, &op)| match op {
            '+' => numbers[i].iter().sum(),
            '*' => numbers[i].iter().product(),
            _ => 0,
        })
        .sum::<i64>();
    println!("{}", total);
}
