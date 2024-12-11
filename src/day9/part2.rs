use std::fs;

fn main() {
    let file_path = "input.txt";
    let file_data = fs::read_to_string(file_path).expect("Failed to open file");

    let mut filesystem: Vec<String> = Vec::new();
    let mut id = 0;
    for (i, c) in file_data.chars().enumerate() {
        // Parse character as a number
        let num = match c.to_digit(10) {
            Some(n) => n as usize,
            None => {
                eprintln!("Skipping invalid character: {}", c);
                continue;
            }
        };

        if i % 2 == 0 {
            for _ in 0..num {
                filesystem.push(id.to_string());
            }
            id += 1;
        } else {
            for _ in 0..num {
                filesystem.push(".".to_string());
            }
        }
    }

    

    let mut i = filesystem.len() -1;
    // Move non-dots to the left, filling empty slots with dots
    while i > 0 {
        if filesystem[i] == "." {
            i -=1;
            continue;
        }
        let mut file_size = 0;
        let start = i;

        let file_id = filesystem[i].clone();
        
        

        for j in 0..i {
            if filesystem[i - j] == file_id {
                file_size += 1;
               
            } else {
                break;
            }
        }

        i = i - file_size;
       

       
        
        
        for j in 0..i {
            
            if filesystem[j..j + file_size].iter().all(|x| x == ".") {
                for k in 0..file_size {
                    filesystem[j + k] = filesystem[start - k].clone();
                    filesystem[start - k] = ".".to_string();
                }
                break;
            }
        }

        
    }

    

    // Calculate checksum
    let mut checksum = 0;
    for (i, entry) in filesystem.iter().enumerate() {
        if entry != "." {
            let num = match entry.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Skipping invalid entry: {}", entry);
                    continue;
                }
            };
            checksum += i * num;
        }
    }
    println!("Checksum: {}", checksum);
}
