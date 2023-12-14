use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(PartialEq, Debug)]
enum State {
    Numeric(bool),
    Other,
}

fn get_adj(p: &Point, size: &Point) -> Vec<Point> {
    let mut adj = Vec::new();

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            adj.push(Point {
                x: p.x + dx,
                y: p.y + dy,
            });
        }
    }

    let mut culled = Vec::new();
    for a in adj.iter() {
        if a.x < 0 || a.x >= size.x || a.y < 0 || a.y >= size.y {
            continue;
        }
        culled.push(a.clone());
    }

    culled
}

fn is_symbol(c: &char) -> bool {
    !c.is_digit(10) && c != &'.'
}

fn main() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let size = Point {
        x: width,
        y: height,
    };

    let mut current = Point { x: 0, y: 0 };
    let mut state = State::Other;
    let mut begin = Point { x: 0, y: 0 };
    let mut owner = Point { x: 0, y: 0 };
    let mut lenght = 0i32;
    let mut ans = 0u32;

    let mut gear_idx: HashMap<Point, Vec<u32>> = HashMap::new();

    while current.y != height {
        let x = current.x as usize;
        let y = current.y as usize;
        let c = lines[y][x];

        if c.is_digit(10) {
            if state == State::Other {
                begin = current.clone();
                state = State::Numeric(false);
            }

            lenght += 1;

            if state == State::Numeric(false) {
                for adj in get_adj(&current, &size) {
                    if is_symbol(&lines[adj.y as usize][adj.x as usize]) {
                        state = State::Numeric(true);
                        owner = adj.clone();
                    }
                }
            }
        } else {
            if state == State::Numeric(true) {
                let str = lines[begin.y as usize][begin.x as usize..(begin.x + lenght) as usize]
                    .into_iter()
                    .collect::<String>();
                let num = str.parse::<u32>().unwrap();
                dbg!(&str);
                dbg!(&num);
                dbg!(&current);
                ans += num;

                match gear_idx.get_mut(&owner) {
                    Some(value) => {
                        value.push(num);
                    }
                    None => {
                        gear_idx.insert(owner.clone(), vec![num]);
                    }
                }
            }
            lenght = 0;
            state = State::Other;
        }

        current.x += 1;
        if current.x == width {
            current.x = 0;
            current.y += 1;
        }
    }

    println!("W: {width}, H {height}");
    println!("Part 1: {ans}");

    ans = 0;
    for (_, v) in gear_idx {
        if v.len() > 1 {
            let mut mul = 1;
            v.iter().for_each(|v| mul = mul * v);
            ans += mul;
        }
    }

    println!("Part 2: {ans}");
}
