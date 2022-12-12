use std::fs;

struct RPS<T> {
    rock: T,
    paper: T,
    scissors: T,
}

/// Parses a file and returns a vector of integers grouped into smaller vectors.
fn parse_file(filename: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut groups: Vec<Vec<String>> = Vec::new();
    for group in contents.split("\n") {
        let mut items: Vec<String> = Vec::new();
        for item in group.split_whitespace() {
            items.push(item.to_string());
        }
        groups.push(items);
    }
    groups
}

fn get_round_results(rounds: &Vec<Vec<String>>) -> Vec<Vec<i32>> {

    let scores = RPS {
        rock: 1,
        paper: 2,
        scissors: 3,
    };

    
    let mut total_scores = Vec::new();

    for round in rounds {
        let mut score_list = Vec::new();
        match round[0].as_str() {
            "A" => {
                score_list.push(scores.rock);
            },
            "B" => {
                score_list.push(scores.paper);
            },
            "C" => {
                score_list.push(scores.scissors);
            },
            _ => println!("Invalid input!"),
        }

        match round[1].as_str() {
            "X" => {
                score_list.push(scores.rock);
            },
            "Y" => {
                score_list.push(scores.paper);
            },
            "Z" => {
                score_list.push(scores.scissors);
            },
            _ => println!("Invalid input!"),
        }

        total_scores.push(score_list);

    }

    total_scores
}

fn strategy_guide(rounds: &Vec<Vec<String>>) -> Vec<Vec<i32>> {

    let scores = RPS {
        rock: 1,
        paper: 2,
        scissors: 3,
    };
    
    let mut total_scores = Vec::new();

    for round in rounds {
        let mut score_list = Vec::new();
        match round[0].as_str() {
            "A" => {
                score_list.push(scores.rock);
            },
            "B" => {
                score_list.push(scores.paper);
            },
            "C" => {
                score_list.push(scores.scissors);
            },
            _ => println!("Invalid input!"),
        }

        match round[1].as_str() {
            "X" => {
                score_list.push(0);
            },
            "Y" => {
                score_list.push(3);
            },
            "Z" => {
                score_list.push(6);
            },
            _ => println!("Invalid input!"),
        }

        total_scores.push(score_list);

    }

    total_scores
}

// compare the scores and return the winner
fn compare_scores(scores: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut winners = Vec::new();
    for score in scores {
        if score[0] == 1 && score[1] == 2 || score[0] == 2 && score[1] == 3 || score[0] == 3 && score[1] == 1 {
            winners.push(6 + score[1])
        } else if score[0] == 1 && score[1] == 3 || score[0] == 2 && score[1] == 1 || score[0] == 3 && score[1] == 2 {
            winners.push(0 + score[1])
        } else {
            winners.push(3 + score[1])
        }
    }
    winners
}

fn correct_scores(scores: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut corrected_scores = Vec::new();
    for score in scores {
        let result = score[1];
        let oponent = score[0];

        if oponent == 1 && result == 6 {
            corrected_scores.push(2 + result);
        } else if oponent == 2 && result == 6 {
            corrected_scores.push(3 + result);
        } else if oponent == 3 && result == 6 {
            corrected_scores.push(1 + result);
        } else if oponent == 1 && result == 3 {
            corrected_scores.push(1 + result);
        } else if oponent == 2 && result == 3 {
            corrected_scores.push(2 + result);
        } else if oponent == 3 && result == 3 {
            corrected_scores.push(3 + result);
        } else if oponent == 1 && result == 0 {
            corrected_scores.push(3 + result);
        } else if oponent == 2 && result == 0 {
            corrected_scores.push(1 + result);
        } else if oponent == 3 && result == 0 {
            corrected_scores.push(2 + result);
        } 
    }
    corrected_scores
}

fn main() {
    let items = parse_file("inputs/day2/list.txt");
    let scores = get_round_results(&items);
    let winners = compare_scores(&scores);

    let scores_two = strategy_guide(&items);
    let winners_two = correct_scores(&scores_two);

    let sum_one = aoc_lib::sum(&winners);
    let sum_two = aoc_lib::sum(&winners_two);
    println!("Answer to round #1: {:?}", sum_one);
    println!("Answer to round #2: {:?}", sum_two);
}
