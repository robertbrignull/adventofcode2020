use regex::Regex;

use crate::util::read_lines;

struct Line {
    password: String,
    letter: char,
    p1: usize,
    p2: usize,
}

impl Line {
    fn is_valid_1(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.letter {
                count += 1;
            }
        }
        return count >= self.p1 && count <= self.p2;
    }

    fn is_valid_2(&self) -> bool {
        let c1 = self.password.chars().nth(self.p1 - 1).unwrap();
        let c2 = self.password.chars().nth(self.p2 - 1).unwrap();
        return (c1 == self.letter) != (c2 == self.letter);
    }
}

fn get_input() -> Vec<Line> {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
    let mut input = Vec::new();
    if let Ok(lines) = read_lines("./day2_input") {
        for line in lines {
            if let Ok(ip) = line {
                let cap = re.captures(&ip).unwrap();
                input.push(Line {
                    password: cap[4].to_string(),
                    letter: cap[3].chars().next().unwrap(),
                    p1: cap[1].parse::<usize>().unwrap(),
                    p2: cap[2].parse::<usize>().unwrap(),
                });
            }
        }
    }
    return input;
}

fn part1(input: &Vec<Line>) {
    let mut x = 0;
    for line in input {
        if line.is_valid_1() {
            x += 1;
        }
    }
    println!("{}", x);
}

fn part2(input: &Vec<Line>) {
    let mut x = 0;
    for line in input {
        if line.is_valid_2() {
            x += 1;
        }
    }
    println!("{}", x);
}

pub fn run() {
    let input = get_input();

    part1(&input);
    part2(&input);
}
