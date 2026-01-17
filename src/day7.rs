fn num_beams(data: Vec<String>) -> Vec<i64> {
    let data = data
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut num_beams = vec![0; data[0].len()];
    num_beams[data[0].iter().position(|&c| c == 'S').unwrap()] = 1;
    for line in data.iter().skip(1) {
        let mut new_num_beams = vec![0; line.len()];
        for (i, &c) in line.iter().enumerate() {
            if c == '^' {
                if i > 0 {
                    new_num_beams[i - 1] += num_beams[i];
                }
                if i + 1 < new_num_beams.len() {
                    new_num_beams[i + 1] += num_beams[i];
                }
            } else {
                new_num_beams[i] += num_beams[i];
            }
        }
        num_beams = new_num_beams;
    }
    num_beams
}

pub fn solve1(data: Vec<String>) {
    println!("{}", num_beams(data).iter().filter(|&&x| x > 0).count());
}

pub fn solve2(data: Vec<String>) {
    println!("{}", num_beams(data).iter().sum::<i64>());
}
