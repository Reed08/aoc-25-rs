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
                let mut valid = true;
                let id_str = id.to_string();

                for rep_len in 1..id_str.len() / 2 + 1 {
                    for begin in (rep_len..id_str.len() - rep_len + 1).step_by(rep_len) {
                        if id_str[begin - rep_len..begin] != id_str[begin..begin + rep_len] {
                            break;
                        }
                        if begin == id_str.len() - rep_len {
                            valid = false;
                            break;
                        }
                    }
                }

                if !valid {
                    // println!("Invalid: {}", id_str);
                    invalid_sum += id;
                } else {
                    // println!("Valid: {}", id_str);
                }
            }
        }
    }

    println!("{}", invalid_sum);
}