use std::collections::HashMap;

fn read_input(filename: &str) -> Vec<i32> {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut nums = Vec::new();
    for line in contents.lines() {
        let mut first: i32 = 0;
        let mut last: i32 = 0;
        let mut first_found = false;
        for c in line.chars() {
            if c.is_digit(10) {
                if !first_found {
                    first = (c as i32 - '0' as i32) * 10;
                    first_found = true;
                }
                last = c as i32 - '0' as i32;
            }
        }
        nums.push(first + last);
    }
    nums
}

fn tok(pos: usize, line: &str) -> (&str, i32, usize) {
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
    for (tok, val) in tokens.iter() {
        if line.len() - pos >= tok.len() && line[pos..pos + tok.len()] == **tok {
            return ("num", *val, tok.len());
        }
    }
    ("unknown", 0, 1)
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut first: i32 = 0;
    let mut last: i32 = 0;
    let mut first_found = false;
    let mut i = 0;
    while i < line.len() {
        let (tok_type, tok_val, tok_len) = tok(i, line);
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

fn parse_input(filename: &str) -> Vec<i32> {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut nums = Vec::new();
    for line in contents.lines() {
        let (first, last) = parse_line(line);
        nums.push(first + last);
    }
    nums
}

fn solve(parser: fn(&str) -> Vec<i32>, filename: &str) {
    let nums = parser(filename);
    let mut total = 0;
    for num in nums {
        total += num;
    }
    println!("{}", total);
}

fn main() {
    solve(read_input, std::env::args().nth(1).unwrap().as_str());
    solve(parse_input, std::env::args().nth(1).unwrap().as_str());
}
