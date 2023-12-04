use anyhow::Result;
use std::fs;
use std::collections::HashMap;

fn parse_cubes(line: &str) -> Vec<Vec<(u32, &str)>> {
    line
        .split(":")
        .nth(1)
        .unwrap()
        .split(";")
        .map(|set| {
            set.trim()
                .split(",")
                .map(|cube| {
                    let parts = cube.trim().split_whitespace().collect::<Vec<_>>();
                    let count = parts[0].parse::<u32>().unwrap();
                    let color = parts[1];

                    (count, color)
                })
            .collect::<Vec<_>>()
        })
    .collect::<Vec<_>>()
}

fn part_one(input: &str) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum_of_possible_game_ids = 0;

    input
        .lines()
        .for_each(|line| {
            let game_id = line
                .split(":")
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let sets_of_cubes = parse_cubes(line);

            let mut is_possible = true;

            for set in &sets_of_cubes {
                let mut color_counts = HashMap::new();

                for &(count, color) in set {
                    *color_counts.entry(color).or_insert(0) += count;
                }

                if *color_counts.get("red").unwrap_or(&0) > max_red ||
                   *color_counts.get("green").unwrap_or(&0) > max_green ||
                   *color_counts.get("blue").unwrap_or(&0) > max_blue {
                    is_possible = false;
                    break;
                }
            }

            if is_possible {
                sum_of_possible_game_ids += game_id;
            }
        });

    sum_of_possible_game_ids
}

fn part_two(input: &str) -> u32 {
    let mut total_power = 0;

    input
        .lines()
        .for_each(|line| {
            let sets_of_cubes = parse_cubes(line);

            let mut min_cubes = HashMap::new();

            for set in &sets_of_cubes {
                for &(count, color) in set {
                    let entry = min_cubes.entry(color).or_insert(0);
                    *entry = (*entry).max(count);
                }
            }

            let red_cubes = *min_cubes.get("red").unwrap_or(&0);
            let green_cubes = *min_cubes.get("green").unwrap_or(&0);
            let blue_cubes = *min_cubes.get("blue").unwrap_or(&0);

            let power = red_cubes * green_cubes * blue_cubes;
            total_power += power;
        });

    total_power
}

fn main() -> Result<()> {
    let input1 = fs::read_to_string("inputs/day2")?;

    let part_one_result = part_one(&input1);
    let part_two_result = part_two(&input1);

    println!("Sum of IDs of possible games: {}", part_one_result);
    println!("Total power: {}", part_two_result);

    Ok(())
}
