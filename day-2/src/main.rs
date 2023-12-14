use std::fmt::Debug;

#[derive(Debug)]

struct Game {
    id: u32,
    rolls: Vec<(u32, u32, u32)>,
}

fn parse_digits(s: &str) -> u32 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn parse_input(input: &Vec<String>) -> Vec<Game> {
    let mut parsed = Vec::new();

    for line in input {
        let s = line
            .split(':')
            .collect::<Vec<&str>>()
            .iter()
            .flat_map(|s| s.split(';').collect::<Vec<&str>>())
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        println!("{:?}", s);

        let mut game = Game {
            id: parse_digits(&s[0]),
            rolls: Vec::new(),
        };


        for roll in s[1..].iter() {
            let colors = roll
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .flat_map(|s| s.split(':').collect::<Vec<&str>>())
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            let mut values = (0u32, 0u32, 0u32);
            for color in colors {
                match color.find("red") {
                    Some(_) => values.0 += parse_digits(&color),
                    None => (),
                }
                match color.find("green") {
                    Some(_) => values.1 += parse_digits(&color),
                    None => (),
                }

                match color.find("blue") {
                    Some(_) => values.2 += parse_digits(&color),
                    None => (),
                }
            }

            game.rolls.push(values);
        }

        parsed.push(game);
    }
    parsed
}

fn solve1(games: &Vec<Game>) {
    let mut ans = 0u32;
    for game in games {
        let is_valid = game
            .rolls
            .iter()
            .all(|roll| roll.0 <= 12 && roll.1 <= 13 && roll.2 <= 14);
        if is_valid {
            ans += game.id;
        }
    }

    println!("Part 1: {ans}");
}

fn solve2(games: &Vec<Game>) {
    let mut ans = 0u32;

    for game in games {
        let mut max = (0, 0, 0);
        for roll in &game.rolls {
            max.0 = std::cmp::max(roll.0, max.0);
            max.1 = std::cmp::max(roll.1, max.1);
            max.2 = std::cmp::max(roll.2, max.2);
        }

        ans += max.0 * max.1 * max.2;
    }

    println!("Part 2: {ans}");
}

fn main() {
    let input: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap()).collect();

    let parsed_games = parse_input(&input);
    solve1(&parsed_games);
    solve2(&parsed_games);
}
