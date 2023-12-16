use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Ray {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Direction {
    fn reflect(self, c: char) -> Direction {
        match c {
            '/' => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            '\\' => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            _ => panic!(),
        }
    }
}

fn raytrace(
    x: usize,
    y: usize,
    dir: Direction,
    grid: &Vec<Vec<char>>,
    vis: &mut Vec<Vec<char>>,
    processed: &mut HashSet<Ray>,
) {
    let mut ray = Ray { x, y, dir };

    loop {
        if ray.x >= grid[0].len() || ray.y >= grid.len() {
            return;
        }

        if processed.contains(&ray) {
            return;
        }

        processed.insert(ray);

        vis[ray.y][ray.x] = '#';

        match grid[ray.y][ray.x] {
            '.' => {}
            '-' => match ray.dir {
                Direction::Up | Direction::Down => {
                    raytrace(ray.x - 1, ray.y, Direction::Left, grid, vis, processed);
                    raytrace(ray.x + 1, ray.y, Direction::Right, grid, vis, processed);
                    return;
                }
                _ => {}
            },
            '|' => match ray.dir {
                Direction::Left | Direction::Right => {
                    raytrace(ray.x, ray.y - 1, Direction::Up, grid, vis, processed);
                    raytrace(ray.x, ray.y + 1, Direction::Down, grid, vis, processed);
                    return;
                }
                _ => {}
            },
            '\\' => {
                let new_dir = ray.dir.reflect('\\');
                match new_dir {
                    Direction::Up => raytrace(ray.x, ray.y - 1, new_dir, grid, vis, processed),
                    Direction::Down => raytrace(ray.x, ray.y + 1, new_dir, grid, vis, processed),
                    Direction::Left => raytrace(ray.x - 1, ray.y, new_dir, grid, vis, processed),
                    Direction::Right => raytrace(ray.x + 1, ray.y, new_dir, grid, vis, processed),
                };
                return;
            }
            '/' => {
                let new_dir = ray.dir.reflect('/');
                match new_dir {
                    Direction::Up => raytrace(ray.x, ray.y - 1, new_dir, grid, vis, processed),
                    Direction::Down => raytrace(ray.x, ray.y + 1, new_dir, grid, vis, processed),
                    Direction::Left => raytrace(ray.x - 1, ray.y, new_dir, grid, vis, processed),
                    Direction::Right => raytrace(ray.x + 1, ray.y, new_dir, grid, vis, processed),
                };
                return;
            }
            _ => panic!(),
        }

        match ray.dir {
            Direction::Up => ray.y = ray.y.wrapping_sub(1),
            Direction::Down => ray.y += 1,
            Direction::Left => ray.x = ray.x.wrapping_sub(1),
            Direction::Right => ray.x += 1,
        };
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let mut vis = vec![vec!['.'; grid[0].len()]; grid.len()];

    let mut processed = HashSet::new();
    raytrace(0, 0, Direction::Right, &grid, &mut vis, &mut processed);
    let ans = vis
        .iter()
        .fold(0, |acc, l| acc + l.iter().filter(|&&c| c == '#').count());

    println!("Part 1: {}", ans);

    let mut ans = 0;

    for x in 0..grid[0].len() {
        let mut vis = vec![vec!['.'; grid[0].len()]; grid.len()];
        let mut processed = HashSet::new();
        raytrace(x, 0, Direction::Down, &grid, &mut vis, &mut processed);
        ans = std::cmp::max(
            ans,
            vis.iter()
                .fold(0, |acc, l| acc + l.iter().filter(|&&c| c == '#').count()),
        );

        let mut vis = vec![vec!['.'; grid[0].len()]; grid.len()];
        let mut processed = HashSet::new();
        raytrace(
            x,
            grid.len() - 1,
            Direction::Down,
            &grid,
            &mut vis,
            &mut processed,
        );
        ans = std::cmp::max(
            ans,
            vis.iter()
                .fold(0, |acc, l| acc + l.iter().filter(|&&c| c == '#').count()),
        );
    }

    for y in 0..grid.len() {
        let mut vis = vec![vec!['.'; grid[0].len()]; grid.len()];
        let mut processed = HashSet::new();
        raytrace(0, y, Direction::Right, &grid, &mut vis, &mut processed);
        ans = std::cmp::max(
            ans,
            vis.iter()
                .fold(0, |acc, l| acc + l.iter().filter(|&&c| c == '#').count()),
        );

        let mut vis = vec![vec!['.'; grid[0].len()]; grid.len()];
        let mut processed = HashSet::new();
        raytrace(
            grid[0].len() - 1,
            y,
            Direction::Left,
            &grid,
            &mut vis,
            &mut processed,
        );
        ans = std::cmp::max(
            ans,
            vis.iter()
                .fold(0, |acc, l| acc + l.iter().filter(|&&c| c == '#').count()),
        );
    }

    println!("Part 2: {}", ans);
}
