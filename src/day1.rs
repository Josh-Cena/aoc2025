pub fn solve1(data: Vec<String>) {
    let mut pos = 50;
    let mut count = 0;
    for line in data {
        let dir = &line[0..1];
        let dist: i32 = line[1..].parse().unwrap();
        match dir {
            "L" => pos = (((pos - dist) % 100) + 100) % 100,
            "R" => pos = (pos + dist) % 100,
            c => panic!("Unknown direction: {}", c),
        }
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
        match dir {
            "L" => {
                if pos == 0 {
                    count += dist / 100;
                } else if dist >= pos {
                    count += (dist - pos) / 100 + 1;
                }
                pos = (((pos - dist) % 100) + 100) % 100;
            },
            "R" => {
                count += (pos + dist) / 100;
                pos = (pos + dist) % 100;
            },
            c => panic!("Unknown direction: {}", c),
        }
    }
    println!("{count}");
}
