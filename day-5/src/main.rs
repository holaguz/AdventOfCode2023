use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct AocMap {
    dest: u64,
    src: u64,
    len: u64,
}

impl AocMap {
    fn begin(&self) -> u64 {
        self.src
    }
    fn end(&self) -> u64 {
        self.src + self.len
    }
}

fn find_lowest(seeds: &Vec<u64>, maps: &Vec<Vec<AocMap>>) -> u64 {
    let mut ans = Vec::new();

    for seed in seeds {
        let mut loc = seed.clone();

        for map in maps {
            for &vec in map {
                if loc >= vec.src && loc < vec.src + vec.len {
                    loc = loc - &vec.src + &vec.dest;
                    break;
                } else {
                    continue;
                }
            }
        }

        ans.push(loc);
    }

    ans.iter().min().unwrap().clone()
}

fn main() {
    let mut lines = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    lines.retain(|l| !l.is_empty());

    let seeds = lines[0].split(' ').collect::<Vec<&str>>()[1..]
        .to_vec()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut key_idxs = Vec::new();
    for (i, l) in lines[1..].iter().enumerate() {
        if l.contains(':') {
            key_idxs.push(i + 1);
        }
    }

    let mut maps: Vec<Vec<AocMap>> = Vec::new();
    maps.clear();

    for k in key_idxs {
        let mut local_maps = Vec::new();
        for l in &lines[k + 1..] {
            if l.contains(':') {
                break;
            }

            let numbers = l
                .split(' ')
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            local_maps.push(AocMap {
                dest: numbers[0],
                src: numbers[1],
                len: numbers[2],
            });
        }
        maps.push(local_maps);
    }

    let p1 = find_lowest(&seeds, &maps);
    println!("Part 1: {:?}", p1);

    let mut intervals = VecDeque::new();

    for chunk in seeds.chunks(2) {
        let src = chunk[0];
        let len = chunk[1];
        intervals.push_back(AocMap { src, len, dest: 0 });
    }

    let mut new = Vec::new();
    for (i, map) in maps.iter().enumerate() {
        while intervals.len() > 0 {
            let iv = intervals.pop_front().unwrap();
            let mut found_mapping = false;
            for m in map {
                let beg = std::cmp::max(m.begin(), iv.begin());
                let end = std::cmp::min(m.end(), iv.end());
                if end > beg {
                    found_mapping = true;

                    // center interval
                    let len = end - beg;
                    let mapped = AocMap {
                        src: beg + m.dest - m.src,
                        len,
                        dest: 0,
                    };
                    new.push(mapped);

                    // left-half, process again
                    if beg > iv.begin() {
                        let len = beg - iv.begin();
                        let mapped = AocMap {
                            src: iv.begin(),
                            len,
                            dest: 0,
                        };
                        intervals.push_back(mapped);
                    }

                    // right-half, process again
                    if iv.end() > end {
                        let len = iv.end() - end;
                        let mapped = AocMap {
                            src: end,
                            len,
                            dest: 0,
                        };
                        intervals.push_back(mapped);
                    }
                }
            }

            // didn't found a map, the interval stays the same
            if !found_mapping {
                new.push(iv);
            }
        }

        new.sort();
        new.dedup();

        println!("Map {} -> {} unique intervals", i, new.len());
        intervals = new.clone().into();
        new.clear();
    }

    let mut intervals: Vec<AocMap> = intervals.into();

    intervals.sort_by(|a, b| a.src.partial_cmp(&b.src).unwrap());

    println!("Part 2: {}", intervals[0].src);
}
