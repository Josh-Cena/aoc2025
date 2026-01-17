pub fn solve1(data: Vec<String>) {
    let mut total = 0;
    for line in data {
        let chars: Vec<char> = line.chars().collect();
        let (max_ind, max1) = chars[..chars.len() - 1]
            .iter()
            .enumerate()
            .max_by_key(|&(i, c)| (*c, -(i as isize)))
            .unwrap();
        let max2 = chars[max_ind + 1..].iter().max().unwrap();
        total += (*max1 as u64 - '0' as u64) * 10 + (*max2 as u64 - '0' as u64);
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
            let (max_ind, max_char) = chars[bound..chars.len() - 11 + i]
                .iter()
                .enumerate()
                .max_by_key(|&(i, c)| (*c, -(i as isize)))
                .unwrap();
            num = num * 10 + (*max_char as u64 - '0' as u64);
            bound += max_ind + 1;
        }
        total += num;
    }
    println!("{}", total);
}
