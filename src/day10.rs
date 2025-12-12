use good_lp::{Expression, Solution, SolverModel, solvers::microlp::microlp, variable, variables};
use regex::Regex;
use std::collections::{HashSet, VecDeque};

pub fn solve1(data: Vec<String>) {
    let re = Regex::new(r"\[([^\]]+)\] ((?:\((?:\d+,)*\d+\) )+)\{((?:\d+,)*\d+)\}").unwrap();
    let lines = data
        .iter()
        .map(|line| {
            let caps = re
                .captures(line)
                .expect(format!("Line didn't match: {}", line.as_str()).as_str());
            let lights = &caps[1];
            let wirings_str = &caps[2];
            let joltage_str = &caps[3];

            let wirings: Vec<Vec<usize>> = wirings_str
                .trim()
                .split(' ')
                .map(|p| {
                    p[1..p.len() - 1]
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect();

            let joltage: Vec<i32> = joltage_str
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            (lights.to_string(), wirings, joltage)
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    'machines: for (lights, wirings, _) in lines {
        let start = std::iter::repeat_n('.', lights.len()).collect::<String>();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start.clone(), 0));
        visited.insert(start.clone());
        while !queue.is_empty() {
            let (cur_st, dist) = queue.pop_front().unwrap();
            for wiring in &wirings {
                let mut next_st = cur_st.chars().collect::<Vec<char>>();
                for &i in wiring {
                    next_st[i] = if next_st[i] == '.' { '#' } else { '.' };
                }
                let next_st_str = next_st.iter().collect::<String>();
                if lights == next_st_str {
                    total += dist + 1;
                    continue 'machines;
                }
                if !visited.contains(&next_st_str) {
                    visited.insert(next_st_str.clone());
                    queue.push_back((next_st_str, dist + 1));
                }
            }
        }
        unreachable!();
    }
    println!("{}", total);
}

fn min_presses(buttons: &[Vec<usize>], target: &[i32]) -> Vec<i32> {
    let mut vars = variables!();
    let mut x_vars = Vec::with_capacity(buttons.len());

    for (j, affected_counters) in buttons.iter().enumerate() {
        let var = vars.add(
            variable()
                .integer()
                .min(0)
                // Can at most press this number of times
                .max(affected_counters.iter().map(|&i| target[i]).min().unwrap())
                .name(format!("x_{}", j)),
        );
        x_vars.push(var);
    }

    let mut objective = Expression::default();
    for &x in &x_vars {
        objective.add_mul(1.0, x);
    }

    let mut model = microlp(vars.minimise(objective));

    for i in 0..target.len() {
        let mut expr = Expression::default();
        for (j, affected_counters) in buttons.iter().enumerate() {
            if affected_counters.contains(&i) {
                expr.add_mul(1.0, x_vars[j]);
            }
        }
        model = model.with(expr.eq(target[i] as f64));
    }

    let solution = model.solve().unwrap();
    x_vars
        .iter()
        .map(|&var| solution.value(var).round() as i32)
        .collect()
}

pub fn solve2(data: Vec<String>) {
    let lines = data
        .iter()
        .map(|line| {
            let caps = Regex::new(r"\[([^\]]+)\] ((?:\((?:\d+,)*\d+\) )+)\{((?:\d+,)*\d+)\}")
                .unwrap()
                .captures(line)
                .expect(format!("Line didn't match: {}", line.as_str()).as_str());
            let lights = &caps[1];
            let wirings_str = &caps[2];
            let joltage_str = &caps[3];

            let wirings: Vec<Vec<usize>> = wirings_str
                .trim()
                .split(' ')
                .map(|p| {
                    p[1..p.len() - 1]
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect();

            let joltage: Vec<i32> = joltage_str
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            (lights.to_string(), wirings, joltage)
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    for (_, wirings, joltage) in lines {
        let presses = min_presses(&wirings, &joltage);
        total += presses.iter().sum::<i32>();
    }
    println!("{}", total);
}
