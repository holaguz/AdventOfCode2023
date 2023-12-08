use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

fn lcm(a: u64, b: u64) -> u64 {
    let mut u = a;
    let mut v = b;
    if v == 0 {
        return u;
    }
    loop {
        u %= v;
        if u == 0 {
            return (a / v) * b;
        }
        v %= u;
        if v == 0 {
            return (a / u) * b;
        }
    }
}

fn lcm_vec(v: &Vec<u64>) -> u64 {
    let mut res = lcm(v[0], v[1]);

    for x in v[2..].iter() {
        res = lcm(*x, res);
    }
    res
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut instructions: VecDeque<char> = lines.first().unwrap().chars().collect();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start_nodes: Vec<&str> = Vec::new();
    for line in &lines[2..] {
        let cur = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(cur, (left, right));
        if cur.chars().last().unwrap() == 'A' {
            start_nodes.push(cur);
        }
    }

    let mut current = "AAA";
    let mut steps = 0;

    while current != "ZZZ" {
        let i = instructions.pop_front().unwrap();
        instructions.push_back(i);

        steps += 1;

        match i {
            'L' => current = map.get(current).unwrap().0,
            'R' => current = map.get(current).unwrap().1,
            _ => (),
        }
    }

    println!("Part 1: {}", steps);

    let mut period = Vec::new();

    for u in start_nodes {
        steps = 0;
        current = u;

        let mut local_ins = instructions.clone();

        while !current.ends_with('Z') {
            let i = local_ins.pop_front().unwrap();
            local_ins.push_back(i);
            steps += 1;
            match i {
                'L' => current = map.get(current).unwrap().0,
                'R' => current = map.get(current).unwrap().1,
                _ => unreachable!(),
            }
        }
        period.push(steps);
    }

    let steps = lcm_vec(&period);
    println!("Part 2: {}", steps);
}
