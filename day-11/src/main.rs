use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut is_big_v: HashSet<usize> = HashSet::new();
    let mut is_big_h: HashSet<usize> = HashSet::new();
    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..height {
        is_big_v.insert(i);
    }

    for i in 0..width {
        is_big_h.insert(i);
    }

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert((x, y));
                is_big_h.remove(&x);
                is_big_v.remove(&y);
            }
        }
    }

    let mut ans: u64 = 0;
    let mut expand: u64 = 0;

    for g1 in &galaxies {
        for g2 in &galaxies {
            for x in g1.0..g2.0 {
                match is_big_h.contains(&x) {
                    true => expand += 1,
                    false => ans += 1,
                }
            }

            for y in g1.1..g2.1 {
                match is_big_v.contains(&y) {
                    true => expand += 1,
                    false => ans += 1,
                }
            }
        }
    }

    println!("Part 1: {}", ans + 2 * expand);
    println!("Part 2: {}", ans + 1_000_000 * expand);
}
