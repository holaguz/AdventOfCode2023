const TEST_INPUT : &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

const MAP: [(&str, u32); 10] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("zero", 0),
];

fn main() {

    let mut input = TEST_INPUT.to_string().replace('\n', " ");
    // let mut input = std::fs::read_to_string("input").unwrap().replace('\n', " ");

    println!("{}\n", input);

    let test = input.matches("two");
    println!("{test:?}\n");

    MAP.iter().for_each(|s| {
        input = input.replace(s.0, &s.1.to_string());
    });
    
    println!("{}\n", input);


    let split: String = input.chars().filter(|c| c.is_numeric() || *c == ' ' ).collect();
    println!("{split:?}\n");

    let x: Vec<u32>  = split.split_whitespace().map(|s| { 
        s.chars().nth(0).unwrap().to_string() + &s.chars().nth_back(0).unwrap().to_string() })
        .map(|s| s.parse::<u32>().unwrap()).collect();

    let part1: u32 = x.iter().sum();

    println!("{}\n", part1);
}
