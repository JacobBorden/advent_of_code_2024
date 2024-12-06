use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut section_two = false;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut sum=0;

    for line in reader.lines() {
        let line = line.expect("Couldn't read line");

        if line.trim().is_empty() {
            section_two = true;
            continue;
        }

        if !section_two {
            let parts: Vec<&str> = line.split("|").collect();
            if parts.len() != 2 {
                eprintln!("Malformed line: {}", line);
                continue;
            }

            let num1 = i32::from_str(parts[0].trim()).expect("Failed to parse first number");
            let num2 = i32::from_str(parts[1].trim()).expect("Failed to parse second number");
            map.entry(num1).or_insert(Vec::new()).push(num2);
        } else if section_two {
            let mut data: Vec<i32> = line
                .split(",")
                .map(|x| i32::from_str(x.trim()).expect("Failed to parse data number"))
                .collect();
            let mut valid_line = true;
            for i in (0..data.len()).rev() {
                for j in (0..i).rev() {
                    if let Some(children) = map.get(&data[i]) {
                        if children.contains(&data[j]) {
                            valid_line = false;
                            data.swap(i,j);
                        }
                    }
                }
                
                }
            
            if !valid_line{
                sum += data[data.len()/2];

            }
        }
    }
    println!("Total sum:{}", sum);
}
