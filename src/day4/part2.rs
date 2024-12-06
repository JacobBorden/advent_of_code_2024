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

    let mut mas_x_count = 0;

    for row in 1..rows.len() - 1 {
        let chars: Vec<char> = rows[row].chars().collect();

        for i in 1..chars.len() - 1 {
            if chars[i] == 'A' {
                // Check all possible positions for 'M' and 'S'
                let up_left_m = row > 0 && i > 0 && rows[row - 1].chars().nth(i - 1) == Some('M');
                let up_right_m = row > 0 && i + 1 < chars.len() && rows[row - 1].chars().nth(i + 1) == Some('M');
                let down_left_m = row + 1 < rows.len() && i > 0 && rows[row + 1].chars().nth(i - 1) == Some('M');
                let down_right_m = row + 1 < rows.len() && i + 1 < chars.len() && rows[row + 1].chars().nth(i + 1) == Some('M');

                let up_left_s = row > 0 && i > 0 && rows[row - 1].chars().nth(i - 1) == Some('S');
                let up_right_s = row > 0 && i + 1 < chars.len() && rows[row - 1].chars().nth(i + 1) == Some('S');
                let down_left_s = row + 1 < rows.len() && i > 0 && rows[row + 1].chars().nth(i - 1) == Some('S');
                let down_right_s = row + 1 < rows.len() && i + 1 < chars.len() && rows[row + 1].chars().nth(i + 1) == Some('S');

                // Check all possible configurations
                if (up_left_m && down_right_s) && (down_left_m && up_right_s) {
                    mas_x_count += 1;
               
                }
                if (up_left_s && down_right_m) && (down_left_m && up_right_s) {
                    mas_x_count += 1;
               
                }
                if (up_left_m && down_right_s) && (down_left_s && up_right_m) {
                    mas_x_count += 1;
               
                }
                if (up_left_s && down_right_m) && (down_left_s && up_right_m) {
                    mas_x_count += 1;
               
                }
            }
        }
    }

    println!("Total MAS-X Count: {}", mas_x_count);
}
