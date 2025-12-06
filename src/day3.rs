pub fn solve1(data: Vec<String>) {
    let mut total = 0;
    for line in data {
        let chars: Vec<char> = line.chars().collect();
        let max = chars[..chars.len()-1].iter().enumerate().rev().max_by_key(|&(_, c)| *c).unwrap();
        let max2 = chars[max.0 + 1..].iter().enumerate().max_by_key(|&(_, c)| *c).unwrap();
        println!("{max:?} {max2:?}");
        total += (*max.1 as u64 - '0' as u64) * 10 + (*max2.1 as u64 - '0' as u64);
    }
    println!("{}", total);
}

pub fn solve2(data: Vec<String>) {
    let mut total = 0;
    for line in data {
        let chars: Vec<char> = line.chars().collect();
        let mut num = 0;
        let mut bound = 0;
        for i in 0..12 {
            let max = chars[bound..chars.len()-11+i].iter().enumerate().rev().max_by_key(|&(_, c)| *c).unwrap();
            num = num * 10 + (*max.1 as u64 - '0' as u64);
            bound += max.0 + 1;
        }
        total += num;
    }
    println!("{}", total);
}
