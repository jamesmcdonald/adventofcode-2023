use std::collections::HashMap;

fn read_input(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

fn tok(pos: usize, line: &str, use_words: bool) -> (&str, i32, usize) {
    let tokens: HashMap<&str, i32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    if line[pos..pos + 1].chars().all(|c| c.is_digit(10)) {
        return ("num", line[pos..pos + 1].parse::<i32>().unwrap(), 1);
    }
    if !use_words {
        return ("unknown", 0, 1);
    }
    for (tok, val) in tokens.iter() {
        if line.len() - pos >= tok.len() && line[pos..pos + tok.len()] == **tok {
            return ("num", *val, tok.len());
        }
    }
    ("unknown", 0, 1)
}

fn parse_line(line: &str, use_words: bool) -> (i32, i32) {
    let mut first: i32 = 0;
    let mut last: i32 = 0;
    let mut first_found = false;
    let mut i = 0;
    while i < line.len() {
        let (tok_type, tok_val, tok_len) = tok(i, line, use_words);
        if tok_type == "num" {
            if !first_found {
                first = tok_val * 10;
                first_found = true;
            }
            last = tok_val;
        }
        i += tok_len;
    }
    (first, last)
}

fn parse_lines(lines: &Vec<String>, use_words: bool) -> Vec<i32> {
    let mut nums = Vec::new();
    for line in lines {
        let (first, last) = parse_line(line, use_words);
        nums.push(first + last);
    }
    nums
}

fn solve(lines: &Vec<String>, use_words: bool) {
    let nums = parse_lines(lines, use_words);
    let mut total = 0;
    for num in nums {
        total += num;
    }
    println!("{}", total);
}

fn main() {
    let lines = read_input(std::env::args().nth(1).unwrap().as_str());
    solve(&lines, false);
    solve(&lines, true);
}
