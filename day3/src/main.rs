use std::{collections::HashMap, env::args};

fn read_input(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(str::to_string)
        .collect()
}

struct Schematic {
    schematic: String,
    pos: usize,
    width: usize,
    starmap: HashMap<usize, u32>,
    gear_ratios: Vec<u32>,
}

impl Schematic {
    fn new(schematic: String, width: usize) -> Self {
        Self {
            schematic,
            pos: width + 1,
            width: width,
            starmap: HashMap::new(),
            gear_ratios: vec![],
        }
    }

    fn get_adjacent_row(&self, pos: usize, up: bool, len: usize) -> Vec<char> {
        let adjacent = self
            .schematic
            .chars()
            .skip(
                if up {
                    pos - (self.width + 2)
                } else {
                    pos + (self.width + 2)
                } - 1,
            )
            .take(len + 2)
            .collect::<Vec<char>>();
        adjacent
    }

    fn is_part(&self, len: usize) -> bool {
        let mut adjacent = vec![];
        adjacent.extend(self.get_adjacent_row(self.pos, true, len));
        let c = self.schematic.chars().nth(self.pos - 1).unwrap();
        adjacent.push(c);
        let c = self.schematic.chars().nth(self.pos + len).unwrap();
        adjacent.push(c);
        adjacent.extend(self.get_adjacent_row(self.pos, false, len));
        adjacent.iter().filter(|c| **c != '.').count() > 0
    }

    fn gear_ratios(&mut self, part: u32, len: usize) {
        let above = self.pos - (self.width + 2) - 1..self.pos - (self.width + 2) + len + 1;
        let below = self.pos + (self.width + 2) - 1..self.pos + (self.width + 2) + len + 1;
        [self.pos - 1, self.pos + len]
            .into_iter()
            .chain(above)
            .chain(below)
            .for_each(|loc| {
                let c = self.schematic.chars().nth(loc).unwrap();
                if c == '*' {
                    match self.starmap.get(&loc) {
                        Some(val) => {
                            self.gear_ratios.push(part * val);
                            println!("{}", self.gear_ratios.iter().sum::<u32>());
                        }
                        None => {
                            self.starmap.insert(loc, part);
                        }
                    }
                }
            });
    }
}

impl Iterator for Schematic {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut part = 0;
        let mut len = 0;
        while self.pos < self.schematic.len() {
            let mut c = self.schematic.chars().nth(self.pos).unwrap();
            if !c.is_digit(10) {
                self.pos += 1;
                continue;
            }
            while c.is_digit(10) {
                part = part * 10 + c.to_digit(10).unwrap();
                len += 1;
                c = self.schematic.chars().nth(self.pos + len).unwrap();
            }
            if self.is_part(len) {
                // Part - increment and return
                self.gear_ratios(part, len);
                self.pos += len;
                return Some(part);
            }
            // Not a part - increment and reset
            self.pos += len;
            part = 0;
            len = 0;
        }
        None
    }
}

fn main() {
    let lines = read_input(args().nth(1).expect("Please provide a filename").as_str());
    let width = lines[0].len();
    let end = ".".repeat(width + 2);
    let lines = lines
        .iter()
        .map(|line| ".".to_owned() + line + ".")
        .collect::<Vec<String>>();
    let lines = [vec![end.clone()], lines, vec![end]].concat();
    let lines = lines.join("");
    // Extract all groups of digits from each line
    // let _ = lines.iter().map(|line| {
    //     let mut res = vec![];
    //     let mut val = 0;
    //     let mut len = 0;
    //     for (i, c) in line.chars().enumerate() {
    //         if c.is_digit(10) {
    //             val = val * 10 + c.to_digit(10).unwrap();
    //             len += 1;
    //             if i == line.len() - 1 {
    //                 res.push((val, i, len));
    //             }
    //         } else if len > 0 {
    //             res.push((val, i, len));
    //             val = 0;
    //             len = 0;
    //         }
    //     }
    //     res
    // });
    let schematic = Schematic::new(lines, width);
    println!("Part 1: {}", schematic.sum::<u32>());
}
