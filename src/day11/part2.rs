use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn count_digits(n: i64) -> usize {
    let s = n.abs().to_string();
    s.len()
}

fn split_number(n: i64) -> (i64, i64) {
    let s = n.to_string();
    let len = s.len();

    // Assuming the number of digits is even
    let half = len / 2;

    let left_str = &s[0..half];
    let right_str = &s[half..];

    let left_num = left_str.parse::<i64>().unwrap();
    let right_num = right_str.parse::<i64>().unwrap();

    (left_num, right_num)
}

fn main() {
    let file_path = "input.txt";
    let file_data = fs::read_to_string(file_path).expect("Failed to open file.");

    let number_string: Vec<&str> = file_data.split_whitespace().collect();
    let mut numbers: HashMap<i64, usize> = HashMap::new();

    // Initialize the hashmap with counts
    for i in number_string {
        let num = i64::from_str(i).expect("Failed to parse string");
        *numbers.entry(num).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut new_numbers: HashMap<i64, usize> = HashMap::new();
        
        for (&number, &count) in &numbers {
            if number == 0 {
                *new_numbers.entry(1).or_insert(0) += count;
            } else if count_digits(number) % 2 == 0 {
                let (left, right) = split_number(number);
                *new_numbers.entry(left).or_insert(0) += count;
                *new_numbers.entry(right).or_insert(0) += count;
            } else {
                let new_number = number * 2024;
                *new_numbers.entry(new_number).or_insert(0) += count;
            }
        }

        numbers = new_numbers;
    }

    let total_count: usize = numbers.values().sum();
    println!("Number of stones: {}", total_count);
}
