extern crate regex;
use regex::Regex;
use std::fs;

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let file_path = "input.txt";
    let fileData = fs::read_to_string(file_path).expect("Failed to open file.");
    let mul = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    let toggle = Regex::new(r"(don't\(\)|do\(\))").expect("Invalid Regex");
    let mut sum = 0;
    let mut enabled = true;

    for mat in mul.find_iter(&fileData) {
        // Check for toggles before the current match
        for toggle_mat in toggle.find_iter(&fileData[..mat.start()]) {
            let command = toggle_mat.as_str();
            enabled = command == "do()";
        }

        if enabled {
            if let Some(captures) = mul.captures(mat.as_str()) {
                let a: i32 = captures[1].parse().expect("invalid first number");
                let b: i32 = captures[2].parse().expect("invalid second number");
                sum += multiply(a, b);
            }
        }
    }

    println!("Total Sum: {}", sum);
}