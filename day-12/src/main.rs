use std::collections::HashMap;
use std::io::{self, BufRead};

fn count(springs: &str, constrains: &Vec<u64>, cache: &mut HashMap<String, u64>) -> u64 {
    if springs.len() < constrains.iter().sum::<u64>() as usize {
        return 0;
    }

    if constrains.is_empty() {
        return match springs.contains('#') {
            true => 0,
            false => 1,
        };
    }

    if springs.len() == constrains[0] as usize && !springs.contains('.') {
        return 1;
    }

    let key = [
        springs.to_string(),
        constrains
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(","),
    ]
    .join(",");

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut ans = 0;

    let curr = springs.chars().next().unwrap();

    if "?.".contains(curr) {
        ans += count(&springs[1..], constrains, cache);
    }

    if "?#".contains(curr) {
        let slice = &springs[..constrains[0] as usize];

        if !slice.contains('.')
            && springs.chars().skip(constrains[0] as usize).next() != Some('#')
            && springs.len() > constrains[0] as usize
        {
            ans += count(
                &springs[constrains[0] as usize + 1..],
                &constrains[1..].to_vec(),
                cache,
            );
        }
    }

    cache.insert(key, ans);
    return ans;
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut springs = Vec::new();
    let mut constraints = Vec::new();

    for l in &lines {
        let splits = l.split(" ").collect::<Vec<&str>>();
        springs.push(splits[0]);
        constraints.push(
            splits[1]
                .split(',')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    let mut ans = 0;
    for (s, c) in springs.iter().zip(constraints.iter()) {
        let mut hashmap = HashMap::new();
        ans += count(s, c, &mut hashmap);
    }
    println!("Part 1: {}", ans);

    ans = 0;
    for (s, c) in springs.iter().zip(constraints.iter()) {
        let s = &[*s].repeat(5).join("?");
        let mut hashmap = HashMap::new();
        ans += count(s, &c.repeat(5), &mut hashmap);
    }

    println!("Part 2: {}", ans);
}
