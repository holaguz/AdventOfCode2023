use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::{self, BufRead};

fn part1(x: i32, y: i32, dx: i32, dy: i32, n: i32, grid: &Vec<Vec<u32>>) -> u32 {
    let (h, w) = (grid.len() as i32, grid[0].len() as i32);

    let mut seen = HashSet::new();
    let mut pq = BinaryHeap::new();

    pq.push((Reverse(0), x, y, dx, dy, n));

    while !pq.is_empty() {
        let (heat, x, y, dx, dy, n) = pq.pop().unwrap();

        if y == h - 1 && x == w - 1 {
            return heat.0;
        }

        if seen.contains(&(x, y, dx, dy, n)) {
            continue;
        }

        seen.insert((x, y, dx, dy, n));

        if n < 3 && (dx, dy) != (0, 0) {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && nx < w && ny >= 0 && ny < h {
                let nh = Reverse(heat.0 + grid[ny as usize][nx as usize]);
                pq.push((nh, nx, ny, dx, dy, n + 1));
            }
        }

        for (ndx, ndy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if (ndx, ndy) != (dx, dy) && (ndx, ndy) != (-dx, -dy) {
                let (nx, ny) = (x + ndx, y + ndy);
                if nx >= 0 && nx < w && ny >= 0 && ny < h {
                    let nh = Reverse(heat.0 + grid[ny as usize][nx as usize]);
                    pq.push((nh, nx, ny, ndx, ndy, 1));
                }
            }
        }
    }
    panic!("No path found");
}

fn part2(x: i32, y: i32, dx: i32, dy: i32, n: i32, grid: &Vec<Vec<u32>>) -> u32 {
    let (h, w) = (grid.len() as i32, grid[0].len() as i32);

    let mut seen = HashSet::new();
    let mut pq = BinaryHeap::new();

    pq.push((Reverse(0), x, y, dx, dy, n));

    while !pq.is_empty() {
        let (heat, x, y, dx, dy, n) = pq.pop().unwrap();
        // dbg!(x, y, dx, dy, n);

        if y == h - 1 && x == w - 1 {
            return heat.0;
        }

        if seen.contains(&(x, y, dx, dy, n)) {
            continue;
        }

        seen.insert((x, y, dx, dy, n));

        if n < 10 && (dx, dy) != (0, 0) {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && nx < w && ny >= 0 && ny < h {
                let nh = Reverse(heat.0 + grid[ny as usize][nx as usize]);
                pq.push((nh, nx, ny, dx, dy, n + 1));
            }
        }

        if n >= 4 || (dx, dy) == (0, 0) {
            for (ndx, ndy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if (ndx, ndy) != (dx, dy) && (ndx, ndy) != (-dx, -dy) {
                    let (nx, ny) = (x + ndx, y + ndy);
                    if nx >= 0 && nx < w && ny >= 0 && ny < h {
                        let nh = Reverse(heat.0 + grid[ny as usize][nx as usize]);
                        pq.push((nh, nx, ny, ndx, ndy, 1));
                    }
                }
            }
        }
    }
    panic!("No path found");
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let ans = part1(0, 0, 0, 0, 0, &grid);
    println!("Part 1: {}", ans);
    let ans = part2(0, 0, 0, 0, 0, &grid);
    println!("Part 2: {}", ans);
}
