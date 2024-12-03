use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main(){
    let file_path = "index.txt";

    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for(index, line) in reader.lines().enumerate(){
        let line = line.expect("failed to read line");

        let parts:Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let num1 = i32::from_str(parts[0]).expect("Failed to parse first number");
            let num2 = i32::from_str(parts[1]).expect("Failed to parse second number");
            col1.push(num1);
            col2.push(num2);
        }
    }
    col1.sort();
    col2.sort();

    for i in 0..col1.len(){
        sum+= (col1[i] - col2[i]).abs();
    }

println!("Sum:{}",sum);
    
}