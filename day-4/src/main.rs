use std::io::{self, BufRead};

#[derive(Debug)]
struct Game {
    id: u32,
    lucky: Vec<u32>,
    numbers: Vec<u32>,
}

fn collect_digits(s: &str, sep: char) -> Vec<u32> {
    let splits = s.split(sep).collect::<Vec<_>>();

    let num_vec = splits
        .iter()
        .map(|s| s.chars().filter(|c| c.is_digit(10)).collect::<String>())
        .collect::<Vec<String>>();

    let non_empty = num_vec.iter().filter(|v| !v.is_empty()).collect::<Vec<_>>();

    let nums = non_empty
        .iter()
        .map(|v| v.parse::<u32>().expect("Not a number"))
        .collect::<Vec<_>>();
    nums
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut games = Vec::new();

    for l in lines {
        let parts = l
            .split(':')
            .collect::<Vec<_>>()
            .iter()
            .flat_map(|v| v.split('|').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let id = collect_digits(parts[0], ' ')[0];
        let lucky = collect_digits(parts[1], ' ');
        let numbers = collect_digits(parts[2], ' ');

        games.push(Game { id, numbers, lucky });
    }

    let mut num_matches = Vec::new();
    for game in &games {
        let mut count = 0;
        for n in &game.numbers {
            if game.lucky.contains(&n) {
                count += 1;
            }
        }
        num_matches.push(count);
    }

    let mut ans = 0;
    for &n in &num_matches {
        if n > 0 {
            ans += 1 << (n - 1);
        }
    }

    println!("Part 1: {:?}", ans);

    let mut count = vec![1; games.len()];

    for (i, matches) in num_matches.iter().enumerate() {
        for j in i + 1..i + 1 + *matches {
            count[j] += count[i];
        }
    }

    println!("Part 2: {:?}", count.iter().sum::<u32>());
}
