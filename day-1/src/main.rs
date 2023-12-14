use std::io::{self, BufRead};

const MAP: [(&str, char); 10] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
    ("zero", '0'),
];

fn process(s: &str) -> u32 {
    let nums = s.chars().filter(|c| c.is_digit(10)).collect::<String>();
    [
        nums.chars().nth(0).unwrap(),
        nums.chars().nth_back(0).unwrap(),
    ]
    .iter()
    .collect::<String>()
    .parse::<u32>()
    .expect("Empty")
}

fn convert(s: &str) -> String {
    let mut new = String::new();
    for i in 0..s.len() {
        MAP.iter().for_each(|(ns, n)| {
            if s[i..].starts_with(ns) {
                new.push(*n);
            } else {
                new.push(s.chars().nth(i).unwrap());
            }
        });
    }
    new
}

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let ans = input.iter().fold(0, |acc, l| acc + process(l));
    println!("Part 1: {}", ans);

    let ans = input.iter().fold(0, |acc, l| acc + process(&convert(l)));
    println!("Part 2: {}", ans);
}
