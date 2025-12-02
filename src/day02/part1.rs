use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut invalid_sum: u64 = 0;
    let input_lines = io::BufReader::new(File::open("src/day02/input.txt").unwrap()).lines();

    for line in input_lines.map_while(Result::ok) {        
        let line = line.trim();

        let ranges = line.split(",");
        for range in ranges {
            // println!("{:#?}", range.split("-").collect::<Vec<_>>());
            let start = range.split("-").nth(0).unwrap().parse::<u64>().unwrap();
            let end = range.split("-").nth(1).unwrap().parse::<u64>().unwrap();

            for id in start..end + 1 {
                let id_str = id.to_string();

                if id_str[..id_str.len() / 2] == id_str[id_str.len() / 2..] {
                    println!("Invalid: {}", id_str);
                    invalid_sum += id;
                }
            }
        }
    }

    println!("{}", invalid_sum);
}