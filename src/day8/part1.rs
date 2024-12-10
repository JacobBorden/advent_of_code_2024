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

    for (key, value) in &map {
        for i in 0..value.len() {
            for j in (i + 1)..value.len() {
                let (row1, col1) = value[i];
                let (row2, col2) = value[j];
                let mut antinode: (isize, isize) = (0, 0);
                let mut antinode2: (isize, isize) = (0, 0);

                if row1 > row2 {
                    let x = row1 - row2;
                    antinode.0 = row2 as isize - x as isize;
                    antinode2.0 = row1 as isize + x as isize;
                } else {
                    let x = row2 - row1;
                    antinode.0 = row1 as isize - x as isize;
                    antinode2.0 = row2 as isize + x as isize;
                }

                if col1 > col2 {
                    let x = col1 - col2;
                    antinode.1 = col2 as isize - x as isize;
                    antinode2.1 = col1 as isize + x as isize;
                } else {
                    let x = col2 - col1;
                    antinode.1 = col1 as isize - x as isize;
                    antinode2.1 = col2 as isize + x as isize;
                }

                if antinode.0 >= 0
                    && antinode.1 >= 0
                    && (antinode.0 as usize) < rows.len()
                    && (antinode.1 as usize) < rows[0].len()
                {
                    additions.push((antinode.0 as usize, antinode.1 as usize));
                }

                if antinode2.0 >= 0
                    && antinode2.1 >= 0
                    && (antinode2.0 as usize) < rows.len()
                    && (antinode2.1 as usize) < rows[0].len()
                {
                    additions.push((antinode2.0 as usize, antinode2.1 as usize));
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

    //Print the resulting count of unique entries
    println!("{}", map[&'#'].len());
}
