use std::{
    collections::{HashSet, VecDeque},
    io::{self, BufRead},
};

fn part1(grid: &Vec<Vec<char>>, sx: usize, sy: usize, steps: usize) {
    let mut q = VecDeque::new();
    let mut step = 0;
    let mut vis: HashSet<(usize, usize)> = HashSet::new();
    q.push_back((sx, sy));

    while step <= steps {
        let mut next = VecDeque::new();
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();

            if vis.contains(&(x, y)) {
                continue;
            }

            vis.insert((x, y));

            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx != 0 && dy != 0 {
                        continue;
                    }

                    let nx = (x as i64 + dx) as usize;
                    let ny = (y as i64 + dy) as usize;

                    if nx >= grid[0].len() || ny >= grid.len() {
                        continue;
                    }

                    if grid[ny][nx] == '#' || vis.contains(&(nx, ny)) {
                        continue;
                    }

                    next.push_back((nx, ny));
                }
            }
        }

        let vis_so_far = vis.iter().count();
        println!(
            "Step {}: reached {} new tiles, visited {}",
            step,
            next.len(),
            vis_so_far
        );
        q = next;
        vis.clear();
        step += 1;
    }
}

fn part2(grid: &Vec<Vec<char>>, sx: usize, sy: usize, steps: usize) {
    let mut q = VecDeque::new();
    let mut step = 0;
    let mut vis: HashSet<(usize, usize)> = HashSet::new();
    q.push_back((sx, sy));

    while step <= steps {
        let mut next = VecDeque::new();
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();

            if vis.contains(&(x, y)) {
                continue;
            }

            vis.insert((x, y));

            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx != 0 && dy != 0 {
                        continue;
                    }

                    let nx = ((x + grid[0].len()) as i64 + dx) as usize;
                    let ny = ((y + grid.len()) as i64 + dy) as usize;

                    let wx = nx as usize % grid[0].len();
                    let wy = ny as usize % grid.len();

                    if grid[wy][wx] == '#' || vis.contains(&(nx, ny)) {
                        continue;
                    }

                    next.push_back((nx, ny));
                }
            }
        }

        let vis_so_far = vis.iter().count();
        println!(
            "Step {}: reached {} new tiles, visited {}",
            step,
            next.len(),
            vis_so_far
        );
        q = next;
        vis.clear();
        step += 1;
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let grid = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // let mut vis = vec![vec![false; grid[0].len()]; grid.len()];

    let sy = grid
        .iter()
        .position(|row| row.iter().any(|&c| c == 'S'))
        .unwrap();
    let sx = grid[sy].iter().position(|&c| c == 'S').unwrap();

    println!("Starting at ({}, {})", sx, sy);
    part1(&grid, sx, sy, 64);
    part2(&grid, sx, sy, 26501365);
}
