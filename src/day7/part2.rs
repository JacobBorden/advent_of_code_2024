use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;


fn concatenate_numbers(num1: i64, num2: i64) -> i64 {
    // Convert numbers to strings
    let num1_str = num1.to_string();
    let num2_str = num2.to_string();

    // Concatenate the strings
    let concatenated_str = num1_str + &num2_str;

    // Convert the concatenated string back to a number
    concatenated_str
        .parse::<i64>()
        .expect("Failed to parse concatenated number")
}


fn is_valid(target: i64, numbers: &[i64], current_value: i64, index: usize) -> bool {
    // Base case: we've processed all numbers
    if index == numbers.len() {
        return current_value == target;
    }

    // Get the current number
    let next_number = numbers[index];
    let concat = concatenate_numbers(current_value, next_number);

    // Try adding and multiplying
    let concat_result = is_valid(target, numbers, concat, index+1);
    let add_result = is_valid(target, numbers, current_value + next_number, index + 1);
    let multiply_result = is_valid(target, numbers, current_value * next_number, index + 1);

    // Return true if any path leads to a valid result
    add_result || multiply_result || concat_result
}

fn is_equation_valid(target: i64, numbers: &[i64]) -> bool {
    if numbers.is_empty() {
        return false;
    }

    // Start from the first number
    is_valid(target, numbers, numbers[0], 1)
}

fn main() {
    let file_name = "input.txt";
    let file = std::fs::File::open(file_name).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.split(':').collect();
        let target = parts[0].trim().parse::<i64>().expect("Failed to parse target");
        let numbers: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|x| x.trim().parse().expect("Failed to parse number"))
            .collect();

        if is_equation_valid(target, &numbers) {
            sum += target;
        }
    }

    println!("Total Calibration Result: {}", sum);
}
