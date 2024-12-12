use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main(){
    let file_path = "test.txt";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut rows = Vec::new();
    for line in reader.lines(){
        let line = line.expect("Failed to read line");
        rows.push(line);
        
    }

    let mut map : HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
    let mut path : HashMap<i32, Vec<(usize, usize)>> = HashMap::new();

    for (row, line) in rows.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let key = c.to_digit(10).unwrap() as i32; 
                map.entry(key).or_insert(Vec::new()).push((row, col));
            
        }
    }

    for i in 0..9{
        if i == 0{
            path.entry(i).or_insert(map[&i].clone());
        }
        for j in 0..path[&i].len(){
            
            for k in 0..map[&(i+1)].len(){
                if ((path[&i][j].0 + 1 == map[&(i+1)][k].0) && (path[&i][j].1 == map[&(i+1)][k].1)) || ((path[&i][j].0 == map[&(i+1)][k].0 +1) && (path[&i][j].1 == map[&(i+1)][k].1)) ||  
                ((path[&i][j].1 + 1 == map[&(i+1)][k].1) && (path[&i][j].0 == map[&(i+1)][k].0)) || ((path[&i][j].1 == map[&(i+1)][k].1 +1) && (path[&i][j].0 == map[&(i+1)][k].0)) {
                    path.entry(i + 1).or_insert(Vec::new()).push(map[&(i+1)][k]);
                }
            }

        }
        
    }
    let end: i32 = 9;
println!("Score:{}",path[&end].len());
}