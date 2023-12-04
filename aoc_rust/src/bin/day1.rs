use std::fs;
use std::io::Result;

fn part_one(input: String) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();

    let numbers = lines
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|x| {
            let first = x.chars().next().unwrap();
            let last = x.chars().next_back().unwrap();
            first.to_string() + &last.to_string()
        })
        .collect::<Vec<_>>();

    let sum = numbers
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .sum::<i32>();

    sum
}

fn part_two(input: String) -> usize {
    let num_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let numbers = input
        .lines()
        .map(|line| {
            let mut char_digits: Vec<char> = vec![];
            let mut temporary = String::new();
            for character in line.chars() {
                temporary += &character.to_string();

                let mut temp_idx = None;
                for (index, spelled_number) in num_words.iter().enumerate() {
                    if temporary.contains(spelled_number) {
                        temp_idx = Some(index);
                        break;
                    }
                }
                if let Some(idx) = temp_idx {
                    let number = idx+ 1;
                    char_digits.push(
                        number
                            .to_string()
                            .chars()
                            .next()
                            .expect("Number should be single-character digit."),
                    );
                    temporary = character.to_string();
                }

                if character.is_ascii_digit() {
                    char_digits.push(character);
                    temporary = String::from("");
                }
            }

            let first_digit = char_digits.first().unwrap_or(&'0').to_owned();
            let last_digit = char_digits.last().unwrap_or(&'0').to_owned();
            let number = format!("{}{}", first_digit, last_digit);
            let number: usize = number.parse().expect("Should parse as a `usize`.");
            number
        })
        .sum();

        numbers

}

fn main() -> Result<()> {
    let input1 = fs::read_to_string("inputs/day1")?;
    let input2 = fs::read_to_string("inputs/day1-part2.test")?;

    let sum_part_one = part_one(input1);
    let sum_part_two = part_two(input2);

    println!("{:?}", sum_part_one);
    println!("{:?}", sum_part_two);

    Ok(())
}
