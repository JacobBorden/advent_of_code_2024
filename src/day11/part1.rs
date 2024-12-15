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
    let fileData = fs::read_to_string(file_path).expect("Failed to open file.");

    let number_string: Vec<&str> = fileData.split_whitespace().collect();
    let mut numbers = Vec::new();

    for i in number_string {
        let num = i64::from_str(i).expect("failed to parse string");
        numbers.push(num);
    }
    for i in 0..25 {
        let mut numbers_temp = Vec::new();
        for number in numbers {
            if number == 0 {
                let num = 1;
                numbers_temp.push(num);
            } else if count_digits(number) % 2 == 0 {
                let num = split_number(number);
                numbers_temp.push(num.0);
                numbers_temp.push(num.1);
            } else {
                let num = number * 2024;
                numbers_temp.push(num);
            }
        }
        numbers = numbers_temp;
        
    }
    println!("Number of stones:{}",numbers.len());
}
