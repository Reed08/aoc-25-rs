use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut total: u64 = 0;
    let input_lines = io::BufReader::new(File::open("src/day03/input.txt").unwrap()).lines();

    for line in input_lines.map_while(Result::ok) {        
        let line = line.trim();

        let mut num = String::from("");
        let mut greatest_idx: usize = 0;
        for digit_num in 0..12 {
            let mut greatest: u8 = 0;
            for digit in greatest_idx..line.len() - (11 - digit_num) {
                if u8::try_from(line.chars().nth(digit).unwrap().to_digit(10).unwrap()).unwrap() > greatest {
                    greatest = u8::try_from(line.chars().nth(digit).unwrap().to_digit(10).unwrap()).unwrap();
                    greatest_idx = digit + 1;
                }
            }
            num += greatest.to_string().as_ref();
        }

        total += num.parse::<u64>().unwrap();
        println!("{}", num);
    }

    println!("{}", total);
}