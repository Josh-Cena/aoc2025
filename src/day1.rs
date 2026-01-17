pub fn solve1(data: Vec<String>) {
    let mut pos = 50;
    let mut count = 0;
    for line in data {
        let dir = &line[0..1];
        let dist: i32 = line[1..].parse().unwrap();
        pos = match dir {
            "L" => (pos - dist).rem_euclid(100),
            "R" => (pos + dist).rem_euclid(100),
            c => panic!("Unknown direction: {}", c),
        };
        if pos == 0 {
            count += 1;
        }
    }
    println!("{count}");
}

pub fn solve2(data: Vec<String>) {
    let mut pos = 50;
    let mut count = 0;
    for line in data {
        let dir = &line[0..1];
        let dist: i32 = line[1..].parse().unwrap();
        pos = match dir {
            "L" => {
                count += (pos - 1i32).div_euclid(100) - (pos - dist - 1).div_euclid(100);
                (pos - dist).rem_euclid(100)
            }
            "R" => {
                count += (pos + dist).div_euclid(100) - pos.div_euclid(100);
                (pos + dist).rem_euclid(100)
            }
            c => panic!("Unknown direction: {}", c),
        };
    }
    println!("{count}");
}
