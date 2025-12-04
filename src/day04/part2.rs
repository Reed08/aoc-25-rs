use std::fs::read_to_string;

fn main() {
    let mut total_possible: u32 = 0;
    let mut last_change: i64 = -1;
    let mut map: Vec<Vec<char>> = read_to_string("src/day04/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    while last_change != 0 {
        let mut possible: u32 = 0;

        for row in 0..map.len() {
            for col in 0..map[row].len() {
                if map[row][col] == '.' {
                    continue;
                }
                let mut surrounding_count: u8 = 0;

                if row != 0 && (map[row - 1][col] == '@' || map[row - 1][col] == 'x') {
                    surrounding_count += 1;
                }
                if row != 0
                    && col + 1 < map[row].len()
                    && (map[row - 1][col + 1] == '@' || map[row - 1][col + 1] == 'x')
                {
                    surrounding_count += 1;
                }
                if col + 1 < map[row].len()
                    && (map[row][col + 1] == '@' || map[row][col + 1] == 'x')
                {
                    surrounding_count += 1;
                }
                if row + 1 < map.len()
                    && col + 1 < map[row].len()
                    && (map[row + 1][col + 1] == '@' || map[row + 1][col + 1] == 'x')
                {
                    surrounding_count += 1;
                }
                if row + 1 < map.len() && (map[row + 1][col] == '@' || map[row + 1][col] == 'x') {
                    surrounding_count += 1;
                }
                if row + 1 < map.len()
                    && col != 0
                    && (map[row + 1][col - 1] == '@' || map[row + 1][col - 1] == 'x')
                {
                    surrounding_count += 1;
                }
                if col != 0 && (map[row][col - 1] == '@' || map[row][col - 1] == 'x') {
                    surrounding_count += 1;
                }
                if row != 0
                    && col != 0
                    && (map[row - 1][col - 1] == '@' || map[row - 1][col - 1] == 'x')
                {
                    surrounding_count += 1;
                }

                if surrounding_count < 4 {
                    possible += 1;
                    map[row][col] = '.';
                }
            }
        }

        last_change = i64::from(possible);
        total_possible += possible;
    }

    // for line in map {
    //     println!("{:?}", line);
    // }
    println!("{}", total_possible);
}
