use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut rows = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        rows.push(line);
    }

    let mut xmas_count = 0;

    for row in 0..rows.len() {
        let chars: Vec<char> = rows[row].chars().collect();

        for i in 0..chars.len() {
            // Horizontal right
            if i + 3 < chars.len() && chars[i] == 'X' && chars[i + 1] == 'M' && chars[i + 2] == 'A' && chars[i + 3] == 'S' {
                xmas_count += 1;
            }

            // Horizontal left
            if i >= 3 && chars[i] == 'X' && chars[i - 1] == 'M' && chars[i - 2] == 'A' && chars[i - 3] == 'S' {
                xmas_count += 1;
            }

            // Vertical down
            if row + 3 < rows.len()
                && rows[row].chars().nth(i) == Some('X')
                && rows[row + 1].chars().nth(i) == Some('M')
                && rows[row + 2].chars().nth(i) == Some('A')
                && rows[row + 3].chars().nth(i) == Some('S')
            {
                xmas_count += 1;
            }

            // Vertical up
            if row >= 3
                && rows[row].chars().nth(i) == Some('X')
                && rows[row - 1].chars().nth(i) == Some('M')
                && rows[row - 2].chars().nth(i) == Some('A')
                && rows[row - 3].chars().nth(i) == Some('S')
            {
                xmas_count += 1;
            }

            // Diagonal (down-right)
            if row + 3 < rows.len()
                && i + 3 < chars.len()
                && rows[row].chars().nth(i) == Some('X')
                && rows[row + 1].chars().nth(i + 1) == Some('M')
                && rows[row + 2].chars().nth(i + 2) == Some('A')
                && rows[row + 3].chars().nth(i + 3) == Some('S')
            {
                xmas_count += 1;
            }

            // Diagonal (up-left)
            if row >= 3
                && i >= 3
                && rows[row].chars().nth(i) == Some('X')
                && rows[row - 1].chars().nth(i - 1) == Some('M')
                && rows[row - 2].chars().nth(i - 2) == Some('A')
                && rows[row - 3].chars().nth(i - 3) == Some('S')
            {
                xmas_count += 1;
            }


            // Diagonal (down-left)
            if row + 3 < rows.len()
                && i >= 3
                && rows[row].chars().nth(i) == Some('X')
                && rows[row + 1].chars().nth(i - 1) == Some('M')
                && rows[row + 2].chars().nth(i - 2) == Some('A')
                && rows[row + 3].chars().nth(i - 3) == Some('S')
            {
                xmas_count += 1;
            }

            // Diagonal (up-right)
            if row >= 3
                && i + 3 < chars.len()
                && rows[row].chars().nth(i) == Some('X')
                && rows[row - 1].chars().nth(i + 1) == Some('M')
                && rows[row - 2].chars().nth(i + 2) == Some('A')
                && rows[row - 3].chars().nth(i + 3) == Some('S')
            {
                xmas_count += 1;
            }
        }
    }

    println!("Total XMAS Count: {}", xmas_count);
}
