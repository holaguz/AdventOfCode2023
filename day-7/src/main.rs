use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Game {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

enum Ruleset {
    Regular,
    Wildcard,
}

#[derive(Debug, Hash, Clone)]
struct Card {
    char: char,
    value: u8,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char)
    }
}

impl Card {
    fn from_char(c: char, ruleset: &Ruleset) -> Card {
        match c {
            'A' => Card { char: c, value: 14 },
            'K' => Card { char: c, value: 13 },
            'Q' => Card { char: c, value: 12 },
            'J' => match ruleset {
                Ruleset::Regular => Card { char: c, value: 11 },
                Ruleset::Wildcard => Card { char: c, value: 1 },
            },
            'T' => Card { char: c, value: 10 },
            _ => Card {
                char: c,
                value: c.to_digit(10).unwrap() as u8,
            },
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Card {}

#[derive(Clone)]
struct Hand {
    cards: Vec<Card>,
    value: u32,
    game: Game,
}

impl Hand {
    fn from_str(s: &str, ruleset: &Ruleset) -> Hand {
        let cards = s
            .chars()
            .map(|c| Card::from_char(c, &ruleset))
            .collect::<Vec<Card>>();
        let value = cards.iter().fold(0u32, |x, c| 99 * x + c.value as u32);
        let game = Hand::parse_game(&cards, &ruleset);
        Hand { cards, value, game }
    }

    fn parse_game(cards: &Vec<Card>, ruleset: &Ruleset) -> Game {
        let mut map: HashMap<char, u8> = HashMap::new();
        for c in cards {
            match map.get(&c.char) {
                Some(x) => map.insert(c.char, x + 1),
                None => map.insert(c.char, 1),
            };
        }

        let jokers = map.get(&'J').unwrap_or(&0).clone();
        let a;
        let b;

        match ruleset {
            Ruleset::Wildcard => {
                map.remove(&'J');
                let mut count = map.into_values().collect::<Vec<u8>>();
                count.sort_by(|a, b| b.cmp(a));
                a = Option::Some(count.get(0).unwrap_or(&0) + jokers);
                b = count.get(1).cloned();
            }
            _ => {
                let mut count = map.into_values().collect::<Vec<u8>>();
                count.sort_by(|a, b| b.cmp(a));
                a = count.get(0).cloned();
                b = count.get(1).cloned();
            }
        }

        match (a, b) {
            (Some(5), _) => Game::FiveOfAKind,
            (Some(4), _) => Game::FourOfAKind,
            (Some(3), Some(2)) => Game::FullHouse,
            (Some(3), _) => Game::ThreeOfAKind,
            (Some(2), Some(2)) => Game::TwoPair,
            (Some(2), _) => Game::Pair,
            (Some(1), _) => Game::HighCard,
            _ => panic!("Invalid hand: {:?}", cards),
        }
    }
}

struct Play {
    hand: Hand,
    stake: u32,
}

fn main() {
    let line = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let hands = line
        .iter()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>()[0])
        .collect::<Vec<&str>>();

    let stakes = line
        .iter()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>()[1])
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let hands_regular: Vec<Hand> = hands
        .iter()
        .map(|h| Hand::from_str(&h, &Ruleset::Regular))
        .collect();

    let mut plays = hands_regular
        .iter()
        .zip(stakes.iter())
        .map(|(h, s)| Play {
            hand: h.clone(),
            stake: *s,
        })
        .collect::<Vec<Play>>();

    plays.sort_by(|a, b| {
        a.hand
            .game
            .cmp(&b.hand.game)
            .then(a.hand.value.cmp(&b.hand.value))
    });

    let p1 = plays
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, p)| acc + (i as u32 + 1) * p.stake);
    println!("Part 1: {}", p1);

    let hands_wildcard: Vec<Hand> = hands
        .iter()
        .map(|h| Hand::from_str(&h, &Ruleset::Wildcard))
        .collect();

    let mut plays = hands_wildcard
        .iter()
        .zip(stakes.iter())
        .map(|(h, s)| Play {
            hand: h.clone(),
            stake: *s,
        })
        .collect::<Vec<Play>>();

    plays.sort_by(|a, b| {
        a.hand
            .game
            .cmp(&b.hand.game)
            .then(a.hand.value.cmp(&b.hand.value))
    });

    let p2 = plays
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, p)| acc + (i as u32 + 1) * p.stake);

    println!("Part 2: {}", p2);
}
