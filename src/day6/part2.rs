use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut start: (i32, i32) = (0, 0);

    let mut rows = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        rows.push(line);
    }

    for (row, line) in rows.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            map.insert((row as i32, col as i32), c);
            if c == '^' {
                start = (row as i32, col as i32);
            }
        }
    }

    // Grid bounds
    let max_row = rows.len() as i32;
    let max_col = rows[0].len() as i32;

    let mut loop_count = 0;

    // Iterate over all positions to add obstructions
    for i in 0..max_row {
        for j in 0..max_col {
            let mut position = start;
            let mut direction = '^';

            // Skip the starting position or existing obstructions
            if (i, j) == start || map.get(&(i, j)) == Some(&'#') {
                continue;
            }

            // Temporarily add obstruction
            map.insert((i, j), '#');

            let mut path: HashMap<(i32, i32), char> = HashMap::new();
            path.insert(position, direction);

            while position.0 >= 0 && position.1 >= 0 && position.0 < max_row && position.1 < max_col {
                match direction {
                    '^' => {
                        let next = (position.0 - 1, position.1);
                        if map.get(&next) == Some(&'#') {
                            direction = '>';
                        } else if let Some(&_) = map.get(&next) {
                            position = next;
                            if path.get(&next) == Some(&direction) {
                                loop_count += 1;
                                break;
                            }
                            path.insert(next, direction);
                        } else {
                            break;
                        }
                    }
                    '>' => {
                        let next = (position.0, position.1 + 1);
                        if map.get(&next) == Some(&'#') {
                            direction = 'v';
                        } else if let Some(&_) = map.get(&next) {
                            position = next;
                            if path.get(&next) == Some(&direction) {
                                loop_count += 1;
                                break;
                            }
                            path.insert(next, direction);
                        } else {
                            break;
                        }
                    }
                    'v' => {
                        let next = (position.0 + 1, position.1);
                        if map.get(&next) == Some(&'#') {
                            direction = '<';
                        } else if let Some(&_) = map.get(&next) {
                            position = next;
                            if path.get(&next) == Some(&direction) {
                                loop_count += 1;
                                break;
                            }
                            path.insert(next, direction);
                        } else {
                            break;
                        }
                    }
                    '<' => {
                        let next = (position.0, position.1 - 1);
                        if map.get(&next) == Some(&'#') {
                            direction = '^';
                        } else if let Some(&_) = map.get(&next) {
                            position = next;
                            if path.get(&next) == Some(&direction) {
                                loop_count += 1;
                                break;
                            }
                            path.insert(next, direction);
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }

            // Remove obstruction
            map.insert((i, j), '.');
        }
    }

    println!("Added obstructions that create loops: {}", loop_count);
}
