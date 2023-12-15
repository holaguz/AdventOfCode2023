use std::io::{self, BufRead};

fn transpose(map: &Vec<String>) -> Vec<String> {
    let width = map[0].len();
    let height = map.len();

    let mut new: Vec<String> = Vec::new();

    for x in 0..width {
        new.push(String::from(""));
        for y in 0..height {
            new.last_mut().unwrap().push(map[y].chars().nth(x).unwrap());
        }
    }
    new
}

fn solve(map: &Vec<String>, part2: bool, skip: Option<usize>) -> Option<usize> {
    let height = map.len();

    let mut pos = None;

    for pivot in 1..height {
        if Some(pivot) == skip {
            continue;
        }

        let n = std::cmp::min(pivot, height - pivot);
        let mut changui = true;
        let mut valid = n > 0;
        for i in 0..n {
            let x = pivot - i - 1;
            let y = pivot + i;

            if map[x] != map[y] {
                if part2 && changui {
                    if map[x]
                        .chars()
                        .zip(map[y].chars())
                        .filter(|(a, b)| a != b)
                        .count()
                        == 1
                    {
                        changui = false;
                        continue;
                    }
                }

                valid = false;
                break;
            }
        }

        if valid {
            pos = Some(pivot);
            break;
        }
    }

    pos
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let maps = lines.split(|l| l.is_empty()).collect::<Vec<_>>();
    println!("Found {} inputs", maps.len());

    let mut ans1 = 0;
    let mut ans2 = 0;
    for (_i, map) in maps.iter().enumerate() {
        let h = solve(&map.to_vec(), false, None);
        let v = solve(&transpose(&map.to_vec()), false, None);
        match (h, v) {
            (Some(h), _) => ans1 += 100 * h,
            (_, Some(v)) => ans1 += v,
            _ => panic!(),
        }

        let h = solve(&map.to_vec(), true, h);
        let v = solve(&transpose(&map.to_vec()), true, v);

        match (h, v) {
            (Some(h), _) => ans2 += 100 * h,
            (_, Some(v)) => ans2 += v,
            _ => panic!(),
        }
    }

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}
