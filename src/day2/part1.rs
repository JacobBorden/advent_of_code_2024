use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main(){

    let file_path = "input.txt";
    let file = File::open(file_path).expect("Could not open file");
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines(){
        let line = line.expect("Failed to read line");
        let mut data = Vec::new();
        let parts:Vec<&str> = line.split_whitespace().collect();
        for i in 0..parts.len(){
            let num = i32::from_str(parts[i]).expect("Failed to parse");
            data.push(num);
        }
        let mut ascending = true;
        let mut descending = true;
        let mut small  = true;
        for i in 0..data.len()-1{
            if data[i] >= data[i+1]{
                ascending = false;
            }
            else if data[i] <= data[i+1]{
                descending = false;

            }
            if ((data[i] - data[i+1]).abs() > 3) || ((data[i] - data[i+1]).abs() < 1) {
                small = false;
            }

        }
        if (ascending || descending) && small{
            sum +=1;
        }
    }

    println!("Sum:{}",sum);
}