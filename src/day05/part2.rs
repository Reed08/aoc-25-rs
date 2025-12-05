use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut fresh: HashSet<u64> = HashSet::new();
    let input_lines = io::BufReader::new(File::open("src/day05/input.txt").unwrap()).lines();

    for line in input_lines.map_while(Result::ok) {
        let line = line.trim().to_string();

        if line == "" {
            break;
        } else {
            let start: u64 = line.split("-").nth(0).unwrap().parse::<u64>().unwrap();
            let end: u64 = line.split("-").nth(1).unwrap().parse::<u64>().unwrap();

            for i in start..=end {
                fresh.insert(i);
            }
        }
    }

    println!("{}", fresh.len());
}
