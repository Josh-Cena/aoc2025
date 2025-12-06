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
    let answers = operators
        .iter()
        .enumerate()
        .map(|(i, &op)| match op {
            "+" => numbers.iter().map(|nums| nums[i]).sum(),
            "*" => numbers.iter().map(|nums| nums[i]).product(),
            _ => 0,
        })
        .collect::<Vec<i64>>();
    let total = answers.iter().sum::<i64>();
    println!("{}", total);
}

pub fn solve2(data: Vec<String>) {
    let mut transposed: Vec<Vec<char>> = Vec::from_iter(vec![Vec::new(); data[0].chars().count()]);
    for line in data.iter() {
        for (i, c) in line.chars().enumerate() {
            transposed[i].push(c);
        }
    }
    let mut total = 0;
    let mut sub_ans = 0;
    let mut op = '+';
    for column in transposed.iter() {
        let (maybe_op, num_chars) = column.split_last().unwrap();
        if !maybe_op.is_whitespace() {
            total += sub_ans;
            op = *maybe_op;
            sub_ans = if op == '+' { 0 } else { 1 };
        }
        let column = num_chars.iter().collect::<String>();
        let column = column.trim();
        if column.is_empty() {
            continue;
        }
        let num = column.parse::<i64>().unwrap();
        if op == '+' {
            sub_ans += num;
        } else {
            sub_ans *= num;
        }
    }
    total += sub_ans;
    println!("{}", total);
}
