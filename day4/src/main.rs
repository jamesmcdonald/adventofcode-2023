use std::{collections::HashMap, env::args};

fn read_input(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(str::to_string)
        .collect()
}

fn getnums(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect::<Vec<u32>>()
}

fn main() {
    let lines = read_input(args().nth(1).unwrap().as_str());
    let cards = lines
        .iter()
        .map(|line| {
            let parts = line.split(":");
            let mut parts = parts.skip(1).next().unwrap().split("|");
            let wins = getnums(parts.next().unwrap());
            let nums = getnums(parts.next().unwrap());
            let matches: u32 = wins
                .iter()
                .filter(|x| nums.contains(*x))
                .count()
                .try_into()
                .unwrap();
            matches
        })
        .collect::<Vec<u32>>();

    let solution = cards
        .iter()
        .map(|&matches| {
            let score = if matches == 0 {
                0
            } else {
                u32::pow(2, matches - 1)
            };
            score
        })
        .sum::<u32>();
    println!("{}", solution);

    let mut copylog = HashMap::new();
    let mut total = 0;
    let _solution2 = cards.iter().enumerate().for_each(|(i, &matches)| {
        let num: u32 = i.try_into().unwrap();
        println!("processing card {}", num);
        let copies: u64 = *copylog.get(&num).unwrap_or(&0);
        println!("there are {} copies", copies);
        let mut count: u64 = 0;
        count += copies + 1;
        for i in num + 1..num + 1 + matches {
            copylog
                .get_mut(&i)
                .map(|x| *x += copies + 1)
                .unwrap_or_else(|| {
                    copylog.insert(i, copies + 1);
                });
        }
        println!("{:?}", copylog);
        println!("{}", count);
        total += count;
    });
    println!("Cards: {}", total);
}
