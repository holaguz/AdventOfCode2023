use std::{io::{self, BufRead}, collections::VecDeque};

enum Op {
    Dash,
    Equal(u32),
}

struct Step<'a> {
    name: &'a str,
    op: Op,
}

fn hash(s: &str) -> u64 {
    let mut ret = 0u64;

    s.bytes().for_each(|c| {
        ret = (17 * (ret + c as u64)) % 256;
    });

    ret
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let inputs: Vec<&str> = lines[0].split(",").collect();

    let ans = inputs.iter().map(|s| hash(s)).sum::<u64>();
    println!("Part 1: {}", ans);

    let steps: Vec<Step> = inputs
        .iter()
        .map(|s| Step {
            name: &s[0..2],
            op: match s.chars().nth(2).unwrap() {
                '-' => Op::Dash,
                '=' => Op::Equal(s.chars().nth(3).unwrap() as u32),
                _ => panic!(),
            },
        })
        .collect();

    let boxes : VecDeque<u32> = vec![0u32; 256].into();
}
