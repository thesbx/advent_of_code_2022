use anyhow::Result;
use std::fs;

fn sum_part_numbers(schematic: &[String]) -> i32 {
    let mut sum = 0;
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for (i, row) in schematic.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c.is_digit(10) && (j == 0 || !row.chars().nth(j - 1).unwrap().is_digit(10)) {
                let number = extract_full_number(schematic, i, j);
                if is_number_adjacent_to_symbol(schematic, i, j, &number.to_string(), &directions) {
                    sum += number;
                }
            }
        }
    }

    sum
}

fn extract_full_number(schematic: &[String], x: usize, y: usize) -> i32 {
    let row = &schematic[x];
    let mut end = y;

    while end < row.len() - 1 && row.chars().nth(end + 1).unwrap().is_digit(10) {
        end += 1;
    }

    row[y..=end].parse::<i32>().unwrap_or(0)
}

fn is_number_adjacent_to_symbol(schematic: &[String], x: usize, y: usize, number: &str, directions: &[(i32, i32)]) -> bool {
    for (idx, _) in number.chars().enumerate() {
        let current_y = y + idx;
        for &(dx, dy) in directions {
            let (new_x, new_y) = (x as i32 + dx, current_y as i32 + dy);
            if new_x >= 0 && new_x < schematic.len() as i32 && new_y >= 0 && new_y < schematic[x].len() as i32 {
                let adjacent_char = schematic[new_x as usize].chars().nth(new_y as usize).unwrap();
                if adjacent_char != '.' && !adjacent_char.is_digit(10) {
                    return true;
                }
            }
        }
    }
    false
}

fn sum_products_of_parts_adjacent_to_stars(schematic: &[String]) -> i32 {
    let mut sum = 0;
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for (i, row) in schematic.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '*' {
                let mut adjacent_parts = Vec::new();
                for &(dx, dy) in &directions {
                    if let Some(part) = get_adjacent_part(schematic, i as i32 + dx, j as i32 + dy) {
                        adjacent_parts.push(part);
                    }
                }

                if adjacent_parts.len() == 2 {
                    sum += adjacent_parts[0] * adjacent_parts[1];
                }
            }
        }
    }

    sum
}

fn get_adjacent_part(schematic: &[String], x: i32, y: i32) -> Option<i32> {
    if x >= 0 && x < schematic.len() as i32 && y >= 0 && y < schematic[x as usize].len() as i32 {
        let char_at_pos = schematic[x as usize].chars().nth(y as usize).unwrap();
        if char_at_pos.is_digit(10) {
            let end = find_number_end(&schematic[x as usize], y as usize);
            let number_str = &schematic[x as usize][y as usize..=end];
            return number_str.parse::<i32>().ok();
        }
    }
    None
}

fn find_number_end(row: &String, start: usize) -> usize {
    let mut end = start;
    while end < row.len() - 1 && row.chars().nth(end + 1).unwrap().is_digit(10) {
        end += 1;
    }
    end
}

fn main() -> Result<()> {
    let test = fs::read_to_string("inputs/day3.test")?;
    let final_input = fs::read_to_string("inputs/day3")?;

    let matrix_test = test.lines().map(|l| l.to_string()).collect::<Vec<String>>();
    let test_sum = sum_part_numbers(&matrix_test);

    let matrix = final_input.lines().map(|l| l.to_string()).collect::<Vec<String>>();
    let final_sum = sum_part_numbers(&matrix);
    let final_product = sum_products_of_parts_adjacent_to_stars(&matrix);

    println!("Sum of adjacent numbers: {}", test_sum);
    println!("Sum of adjacent numbers: {}", final_sum);
    println!("Sum of adjacent parts ratios: {}", final_product);

    Ok(())
}
