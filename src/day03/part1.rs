use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut total: u64 = 0;
    let input_lines = io::BufReader::new(File::open("src/day03/input.txt").unwrap()).lines();

    for line in input_lines.map_while(Result::ok) {        
        let line = line.trim();

        let mut greatest: u32 = 0;
        for dig_1 in 0..line.len() - 1 {
            for dig_2 in dig_1 + 1..line.len() {
                let num = format!("{}{}", line.chars().nth(dig_1).unwrap(), line.chars().nth(dig_2).unwrap()).parse::<u32>().unwrap();

                if num > greatest {
                    greatest = num;
                }
            }
        }

        total += u64::from(greatest);
    }

    println!("{}", total);
}