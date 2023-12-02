use std::{collections::HashMap, env::args};

fn read_input(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(str::to_string)
        .collect()
}

fn total(colour: &str) -> i32 {
    match colour {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => 0,
    }
}

fn parse_line(line: String) -> (i32, i32) {
    let parts = line.split(":").collect::<Vec<&str>>();
    let mut game = parts[0]
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let guesses = parts[1].split(";").collect::<Vec<&str>>();
    let mut maxes: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for guess in guesses {
        let parts = guess.split(",").collect::<Vec<&str>>();
        for part in parts {
            let parts = part.trim().split_whitespace().collect::<Vec<&str>>();
            let count = parts[0].parse::<i32>().unwrap();
            let colour = parts[1];
            if count > total(colour) {
                game = 0;
            }
            if count > maxes[colour] {
                maxes.insert(colour, count);
            }
        }
    }
    (game, maxes.values().product())
}

fn parse_lines(lines: Vec<String>) -> (i32, i32) {
    let mut games = 0;
    let mut powers = 0;
    for line in lines {
        let (game, power) = parse_line(line);
        games += game;
        powers += power;
    }
    (games, powers)
}

fn main() {
    let lines = read_input(args().nth(1).unwrap().as_str());
    let (games, power) = parse_lines(lines);
    println!("{}\n{}", games, power);
}
