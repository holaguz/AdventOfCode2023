use std::io::{self, BufRead};

fn solve(input: &Vec<i64>) -> i64 {
    if input.iter().all(|v| *v == 0) {
        return 0;
    }

    let mut diff = Vec::new();
    let mut prev = input.get(0).unwrap();

    for v in input.iter().skip(1) {
        diff.push(v - prev);
        prev = v;
    }

    return input.last().unwrap() + solve(&diff);
}

fn main() {
    let parsed: Vec<Vec<i64>> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|n| n.parse::<_>().unwrap())
                .collect()
        })
        .collect();

    let p1 = parsed.iter().fold(0, |acc, x| acc + solve(x));
    println!("Part 1: {}", p1);

    let p2 = parsed.iter().fold(0, |acc, x| {
        let rev: Vec<i64> = x.iter().rev().cloned().collect();
        acc + solve(&rev)
    });

    println!("Part 2: {}", p2);
}
