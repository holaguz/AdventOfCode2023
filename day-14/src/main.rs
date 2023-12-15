use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn rotate_ccw(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = platform[0].len();
    let mut new = Vec::new();

    for x in 0..width {
        let col: Vec<char> = platform.iter().map(|r| r[x]).collect();
        new.push(col);
    }

    new.reverse();
    new
}

// ugly
fn rotate_cw(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut platform = platform.clone();
    for _ in 0..3 {
        platform = rotate_ccw(&platform);
    }
    platform
}

fn tilt_west(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut platform = platform.clone();

    // hacky, can do it in linear time
    for l in platform.iter_mut() {
        let mut changed = true;
        while changed {
            changed = false;
            for x in 1..l.len() {
                if ".#".contains(l[x]) {
                    continue;
                }

                if l[x - 1] == '.' {
                    changed = true;
                    l[x - 1] = 'O';
                    l[x] = '.';
                }
            }
        }
    }
    platform
}

fn cycle(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut platform = platform.clone();
    /* north */
    platform = rotate_ccw(&platform);
    platform = tilt_west(&platform);
    platform = rotate_cw(&platform);
    /* west */
    platform = tilt_west(&platform);
    /* south */
    platform = rotate_cw(&platform);
    platform = tilt_west(&platform);
    platform = rotate_ccw(&platform);
    /* east */
    platform = rotate_ccw(&platform);
    platform = rotate_ccw(&platform);
    platform = tilt_west(&platform);
    platform = rotate_ccw(&platform);
    platform = rotate_ccw(&platform);

    platform
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut platform: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let orig = platform.clone();

    let mut cache: HashMap<Vec<Vec<char>>, u32> = HashMap::new();
    cache.insert(platform.clone(), 0);

    platform = rotate_ccw(&platform);
    platform = tilt_west(&platform);
    platform = rotate_cw(&platform);

    let ans = platform.iter().rev().enumerate().fold(0, |acc, (i, l)| {
        acc + (i + 1) * l.iter().filter(|c| **c == 'O').count()
    });
    println!("Part 1: {}", ans);

    let mut i = 1;
    platform = orig;

    loop {
        platform = cycle(&platform);

        match cache.get(&platform) {
            Some(v) => {
                println!("Found loop: {} is the same as {}", i, v);
                break;
            }
            _ => {
                cache.insert(platform.clone(), i);
            }
        }

        i += 1;
    }

    let first = cache.get(&platform).unwrap();
    let delta = i - first;
    let need = (1000000000 - first) % delta;

    println!("delta: {}, need {} more iters", delta, need);

    for _ in 0..need {
        platform = cycle(&platform);
    }

    let ans = platform.iter().rev().enumerate().fold(0, |acc, (i, l)| {
        acc + (i + 1) * l.iter().filter(|c| **c == 'O').count()
    });

    println!("Part 2: {}", ans);
}
