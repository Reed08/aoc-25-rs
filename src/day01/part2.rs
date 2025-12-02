use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut pos: i32 = 50;
    let mut count: u32 = 0;

    let input_lines = io::BufReader::new(File::open("src/day01/input.txt").unwrap()).lines();

    for line in input_lines.map_while(Result::ok) {
        let line: &str = line.trim();
        let mut left = false;

        let num = line[1..].parse::<i32>().unwrap();
        if &line[..1] == "L" {
            left = true;
        }

        if left {
            if num >= pos {
                count += if pos != 0 { u32::try_from(1 + (num - pos) / 100).unwrap() } else { u32::try_from(num / 100).unwrap() };
            }

            pos -= num;

            while pos < 0 {
                pos += 100;
            }
        } else {
            if num >= 100 - pos {
                count += if pos != 0 { u32::try_from(1 + (num - (100 - pos)) / 100).unwrap() } else { u32::try_from(num / 100).unwrap() };
            }

            pos += num;

            while pos > 99 {
                pos -= 100;
            }
        }
    }

    println!("{}", count);
}