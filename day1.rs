use crate::util::read_lines;

fn get_input() -> Vec<u32> {
    let mut input = Vec::new();
    if let Ok(lines) = read_lines("./day1_input") {
        for line in lines {
            if let Ok(ip) = line {
                input.push(ip.parse::<u32>().unwrap())
            }
        }
    }
    return input;
}

fn part1(input: &Vec<u32>) {
    let target = 2020;
    let mut i = 0;
    let mut j = input.len() - 1;
    while i < j {
        let s = input[i] + input[j];
        if s > target {
            j -= 1;
        } else if s < target {
            i += 1;
        } else {
            println!("{}", input[i] * input[j]);
            return;
        }
    }

    println!("Did not find result for part 1");
    std::process::exit(1);
}

fn part2(input: &Vec<u32>) {
    let target = 2020;
    for i in 0 .. input.len() - 2 {
        let mut j = i + 1;
        let mut k = input.len() - 1;
        while j < k {
            let s = input[i] + input[j] + input[k];
            if s > target {
                k -= 1;
            } else if s < target {
                j += 1;
            } else {
                println!("{}", input[i] * input[j] * input[k]);
                return
            }
        }
    }

    println!("Did not find result for part 2");
    std::process::exit(1);
}

pub fn run() {
    let mut input = get_input();
    input.sort();

    part1(&input);
    part2(&input);
}
