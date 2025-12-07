use std::collections::HashMap;

pub fn solve1(data: Vec<String>) {
    let data = data.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let mut has_beam = vec![false; data[0].len()];
    has_beam[data[0].iter().position(|&c| c == 'S').unwrap()] = true;
    let mut count = 0;
    for line in data.iter().skip(1) {
        let mut new_has_beam = has_beam.clone();
        for (i, &c) in line.iter().enumerate() {
            if c == '^' && has_beam[i] {
                if i > 0 {
                    new_has_beam[i - 1] = true;
                }
                new_has_beam[i] = false;
                if i + 1 < new_has_beam.len() {
                    new_has_beam[i + 1] = true;
                }
                count += 1;
            }
        }
        has_beam = new_has_beam;
    }
    println!("{}", count);
}

fn count(data: Vec<Vec<char>>, lvl: usize, beam_pos: usize, cache: &mut HashMap<(usize, usize), i64>) -> i64 {
    if cache.contains_key(&(lvl, beam_pos)) {
        return cache[&(lvl, beam_pos)];
    }
    if lvl == data.len() {
        return 1;
    }
    if data[lvl][beam_pos] != '^' {
        let res = count(data, lvl + 1, beam_pos, cache);
        cache.insert((lvl, beam_pos), res);
        return res;
    }
    let mut total = 0;
    if beam_pos > 0 {
        total += count(data.clone(), lvl + 1, beam_pos - 1, cache);
    }
    if beam_pos + 1 < data[0].len() {
        total += count(data.clone(), lvl + 1, beam_pos + 1, cache);
    }
    cache.insert((lvl, beam_pos), total);
    total
}

pub fn solve2(data: Vec<String>) {
    let data = data.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let start_pos = data[0].iter().position(|&c| c == 'S').unwrap();
    let count = count(data, 1, start_pos, &mut HashMap::new());
    println!("{}", count);
}
