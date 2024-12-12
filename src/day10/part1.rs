use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, VecDeque};

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut rows = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        rows.push(line);
    }

    let height = rows.len();
    let width = if height > 0 { rows[0].len() } else { 0 };

    // Convert input lines to a 2D array of integers (heights)
    let mut grid = Vec::with_capacity(height);
    for r in 0..height {
        let mut row_heights = Vec::with_capacity(width);
        for c in rows[r].chars() {
            // Each character is assumed to be a digit '0' to '9'
            let h = c.to_digit(10).expect("Invalid character in map") as usize;
            row_heights.push(h);
        }
        grid.push(row_heights);
    }

    // Directions for adjacent moves: up, down, left, right
    let directions = [(0,1), (0,-1), (1,0), (-1,0)];

    // Find all trailheads (positions with height = 0)
    let mut trailheads = Vec::new();
    for r in 0..height {
        for c in 0..width {
            if grid[r][c] == 0 {
                trailheads.push((r, c));
            }
        }
    }

    let mut total_score = 0;

    // For each trailhead, find all distinct '9' cells reachable via a strictly incrementing path
    for &start in &trailheads {
        let mut visited = HashSet::new();
        let mut reachable_nines = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some((cr, cc)) = queue.pop_front() {
            let current_height = grid[cr][cc];
            if current_height == 9 {
                // We've reached a '9', record it
                reachable_nines.insert((cr, cc));
                // We don't need to continue from a '9' since there's no digit after 9
                continue;
            }

            // We can move only to tiles with height exactly current_height + 1
            let next_height = current_height + 1;

            for &(dr, dc) in &directions {
                let nr = cr as isize + dr;
                let nc = cc as isize + dc;
                if nr < 0 || nr >= height as isize || nc < 0 || nc >= width as isize {
                    continue; // Out of bounds
                }
                let nr = nr as usize;
                let nc = nc as usize;
                if !visited.contains(&(nr, nc)) && grid[nr][nc] == next_height {
                    visited.insert((nr, nc));
                    queue.push_back((nr, nc));
                }
            }
        }

        // Score for this trailhead is how many distinct '9' positions were found
        total_score += reachable_nines.len();
    }

    println!("Sum of the scores of all trailheads: {}", total_score);
}
