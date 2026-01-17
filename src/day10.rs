use good_lp::{Expression, Solution, SolverModel, solvers::microlp::microlp, variable, variables};
use regex::Regex;
use std::collections::{HashSet, VecDeque};

fn parse_input(data: Vec<String>) -> Vec<(String, Vec<Vec<usize>>, Vec<i32>)> {
    let re = Regex::new(r"\[([^\]]+)\] ((?:\((?:\d+,)*\d+\) )+)\{((?:\d+,)*\d+)\}").unwrap();
    data.iter()
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
        .collect()
}

pub fn solve1(data: Vec<String>) {
    let lines = parse_input(data);
    let mut total = 0;
    'machines: for (lights, wirings, _) in lines {
        let lights_st = lights
            .chars()
            .fold(0, |acc, c| acc * 2 + if c == '#' { 1 } else { 0 });
        let n_lights = lights.len();
        let start = 0;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start, 0));
        visited.insert(start);
        while !queue.is_empty() {
            let (cur_st, dist) = queue.pop_front().unwrap();
            for wiring in &wirings {
                let mut next_st = cur_st;
                for &i in wiring {
                    next_st = next_st ^ (1 << (n_lights - 1 - i));
                }
                if lights_st == next_st {
                    total += dist + 1;
                    continue 'machines;
                }
                if !visited.contains(&next_st) {
                    visited.insert(next_st);
                    queue.push_back((next_st, dist + 1));
                }
            }
        }
        unreachable!();
    }
    println!("{}", total);
}

fn min_presses(buttons: &[Vec<usize>], target: &[i32]) -> i32 {
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

    let mut model = microlp(vars.minimise(&objective));

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
    solution.eval(&objective).round() as i32
}

pub fn solve2(data: Vec<String>) {
    let lines = parse_input(data);
    let total = lines
        .iter()
        .map(|(_, wirings, joltage)| min_presses(&wirings, &joltage))
        .sum::<i32>();
    println!("{}", total);
}
