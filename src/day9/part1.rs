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



    // Move non-dots to the left, filling empty slots with dots
    for i in (0..filesystem.len()).rev() {
        if filesystem[i] != "." {
            for j in 0..i {
                if filesystem[j] == "." {
                    filesystem[j] = filesystem[i].clone();
                    filesystem[i] = ".".to_string();
                    break;
                }
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
