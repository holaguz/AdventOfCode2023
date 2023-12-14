use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let time = lines[0].split_whitespace().skip(1).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let dist = lines[1].split_whitespace().skip(1).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    let mut ans = vec![0; time.len()];

    for i in 0..time.len() {
        let (t, d) = (time[i], dist[i]);
        let l = (t as f64 - (t as f64 * t as f64 - 4.0 * d as f64).sqrt()) / 2.0;
        let r = (t as f64 + (t as f64 * t as f64 - 4.0 * d as f64).sqrt()) / 2.0;

        ans[i] = r as u32 - l as u32;

        if l.fract() == 0.0 {
            ans[i] -= 1;
        }
    }

    let p1 = ans.iter().fold(1, |acc, x| acc * x);
    println!("Part 1: {}", p1);

    /* --- Part Two --- */

    let t = lines[0].chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u64>().unwrap();
    let d = lines[1].chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u64>().unwrap();

    let l = (t as f64 - (t as f64 * t as f64 - 4.0 * d as f64).sqrt()) / 2.0;
    let r = (t as f64 + (t as f64 * t as f64 - 4.0 * d as f64).sqrt()) / 2.0;

    let mut p2 = r as u32 - l as u32;
    if l.fract() == 0.0 {
        p2 -= 1;
    }

    println!("Part 2: {}", p2);
}
