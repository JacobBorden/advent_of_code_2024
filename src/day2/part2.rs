use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
// Function to check if a sequence is safe
fn is_safe(data: &[i32]) -> bool {
    if data.len() < 2 {
        return true; // A single-element or empty sequence is trivially safe
    }
    let mut ascending = true;
    let mut descending = true;
    for i in 0..data.len() - 1 {
        let diff =( data[i + 1] - data[i]).abs();
        if diff < 1 || diff > 3 {
            return false; // Difference out of range
        }
        if data[i] >= data[i + 1] {
            ascending = false; // Not strictly increasing
        }
        if data[i] <= data[i + 1] {
            descending = false; // Not strictly decreasing
        }
    }
    ascending || descending
}
// Main function
fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).expect("Could not open file");
    let reader = io::BufReader::new(file);
    let mut safe_count = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue; // Skip empty lines
        }
        let mut data = Vec::new();
        for part in parts {
            let num = i32::from_str(part).expect("Failed to parse");
            data.push(num);
        }
        if is_safe(&data) {
            safe_count += 1;
            continue; // Already safe, no need to check further
        }
        // Check if removing one level makes the sequence safe
        for i in 0..data.len() {
            let mut temp_data = data.clone();
            temp_data.remove(i); // Remove the ith element
            if is_safe(&temp_data) {
                safe_count += 1;
                break; // Stop checking further; this line is safe
            }
        }
    }
    println!("Safe Reports: {}", safe_count);
}