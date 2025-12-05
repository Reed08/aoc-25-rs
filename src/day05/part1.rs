use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut switch = false;
    let mut fresh: u64 = 0;
    let mut nums: Vec<String> = vec![];
    let input_lines = io::BufReader::new(File::open("src/day05/input.txt").unwrap()).lines();

    for line in input_lines.map_while(Result::ok) {
        let line = line.trim().to_string();

        if switch || line == "" {
            if !switch {
                switch = true;
                continue;
            }
            let num = line.parse::<u64>().unwrap();
            for valid in &nums {
                let start = valid.split("-").nth(0).unwrap().parse::<u64>().unwrap();
                let end: u64 = valid.split("-").nth(1).unwrap().parse::<u64>().unwrap();

                if num >= start && num <= end {
                    fresh += 1;
                    break;
                }
            }
        } else {
            nums.push(line);
        }
    }

    println!("{}", fresh);
}
