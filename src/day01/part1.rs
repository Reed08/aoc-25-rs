use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    let mut pos: i32 = 50;
    let mut count: u32 = 0;

    let input_lines = io::BufReader::new(File::open("src/day01/input.txt").unwrap()).lines();

    for line in input_lines.map_while(Result::ok) {
        let line = line.trim();

        let mut num = line[1..].parse::<i32>().unwrap();
        if &line[..1] == "L" {
            num = -num;
        }

        pos += num;

        while pos > 99 {
            pos -= 100;
        }
        while pos < 0 {
            pos += 100;
        }

        if pos == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}