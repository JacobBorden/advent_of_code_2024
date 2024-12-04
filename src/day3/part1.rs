extern crate regex;
use regex::Regex;
use std::fs;


fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let file_path = "input.txt";
    let fileData = fs::read_to_string(file_path).expect("Failed to open file.");
    let mul  = Regex::new(r"mul\((\d*,\d*)\)").expect("Invalid regex");
    let mut sum = 0;
    for cap in mul.captures_iter(&fileData) {
        let args = &cap[1];

        let parts: Vec<i32> = args
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid number"))
            .collect();
        
        if parts.len() == 2 {
            sum += multiply(parts[0], parts[1]);
        }
    }
    println!("Total Sum:{}", sum);
}
