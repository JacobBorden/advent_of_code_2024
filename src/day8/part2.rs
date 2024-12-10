use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut rows = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        rows.push(line);
    }

    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (row, line) in rows.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                map.entry(c).or_insert(Vec::new()).push((row, col));
            }
        }
    }

    let mut additions: Vec<(usize, usize)> = Vec::new();

    for (_key, value) in &map {
        for i in 0..value.len() {
            for j in (i + 1)..value.len() {
                let (row1, col1) = value[i];
                let (row2, col2) = value[j];

                let mut current_antinode1 = (row1 as isize, col1 as isize);
                let mut current_antinode2 = (row2 as isize, col2 as isize);
                additions.push((current_antinode1.0 as usize, current_antinode1.1 as usize));
                additions.push((current_antinode2.0 as usize, current_antinode2.1 as usize));

                let delta_row = (row2 as isize) - (row1 as isize);
                let delta_col = (col2 as isize) - (col1 as isize);

                while current_antinode1.0 >= 0
                    && current_antinode1.1 >= 0
                    && (current_antinode1.0 as usize) < rows.len()
                    && (current_antinode1.1 as usize) < rows[0].len()
                {
                    let next_antinode1 = (
                        current_antinode1.0 - delta_row,
                        current_antinode1.1 - delta_col,
                    );

                    if next_antinode1.0 >= 0
                        && next_antinode1.1 >= 0
                        && (next_antinode1.0 as usize) < rows.len()
                        && (next_antinode1.1 as usize) < rows[0].len()
                    {
                        additions.push((next_antinode1.0 as usize, next_antinode1.1 as usize));
                    }

                    current_antinode1 = next_antinode1;
                }

                while current_antinode2.0 >= 0
                    && current_antinode2.1 >= 0
                    && (current_antinode2.0 as usize) < rows.len()
                    && (current_antinode2.1 as usize) < rows[0].len()
                {
                    let next_antinode2 = (
                        current_antinode2.0 + delta_row,
                        current_antinode2.1 + delta_col,
                    );

                    if next_antinode2.0 >= 0
                        && next_antinode2.1 >= 0
                        && (next_antinode2.0 as usize) < rows.len()
                        && (next_antinode2.1 as usize) < rows[0].len()
                    {
                        additions.push((next_antinode2.0 as usize, next_antinode2.1 as usize));
                    }

                    current_antinode2 = next_antinode2;
                }
            }
        }
    }

    // Use a HashSet to remove duplicates from the additions
    let mut unique_additions: HashSet<(usize, usize)> = HashSet::new();
    for &(row, col) in &additions {
        unique_additions.insert((row, col));
    }

    // Replace map[&'#'] with unique values
    map.entry('#').or_insert(Vec::new()).clear(); // Clear existing entries for '#'
    for (row, col) in unique_additions {
        map.entry('#').or_insert(Vec::new()).push((row, col));
    }


    // Print the resulting count of unique entries
    println!("{}", map[&'#'].len());
}
