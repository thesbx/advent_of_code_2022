use std::fs;
use std::io::Result;

use itertools::Itertools;

fn main() -> Result<()> {
    let input1 = fs::read_to_string("inputs/day2.test")?;
    let lines = input1
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect_vec()
        })
        .collect_vec();

    let mut total_matches = 0;
    let mut total_matches_with_removal = 0;

    for line in &lines {
        // Check if all adjacent levels differ by at least one and at most three
        let valid_differences = line.windows(2).all(|w| {
            let diff = (w[0] - w[1]).abs();
            diff >= 1 && diff <= 3
        });

        // Rule 1: Total lines matching the original rules
        if (in_order(line.to_vec())) && valid_differences {
            total_matches += 1;
        }

        // Rule 2: Check if removing one level makes it valid
        if matches_with_removal(line) {
            total_matches_with_removal += 1;
        }
    }

    println!("Total lines matching the original rules: {}", total_matches);
    println!(
        "Total lines matching the rules (allowing removal): {}",
        total_matches_with_removal
    );

    Ok(())
}

fn in_order(line: Vec<i32>) -> bool {
    if line.windows(2).all(|w| w[0] < w[1]) || line.windows(2).all(|w| w[0] > w[1]) {
        return true;
    }
    false
}

// Function to check if removing one level makes a line valid
fn matches_with_removal(line: &[i32]) -> bool {
    if line.len() < 3 {
        return false; // Can't remove one level if there are less than 3 levels
    }

    for i in 0..line.len() {
        let modified_line: Vec<i32> = line
            .iter()
            .enumerate()
            .filter_map(|(idx, &val)| if idx != i { Some(val) } else { None })
            .collect();

        let valid_differences = modified_line.windows(2).all(|w| {
            let diff = (w[0] - w[1]).abs();
            diff >= 1 && diff <= 3
        });

        if (in_order(modified_line)) && valid_differences {
            return true;
        }
    }

    false
}
