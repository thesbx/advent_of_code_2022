use std::fs;

fn parse_file(filename: &str) -> Vec<Vec<Vec<i32>>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut groups: Vec<Vec<Vec<i32>>> = Vec::new();
    for group in contents.split("\n") {
        let mut items: Vec<Vec<i32>> = Vec::new();
        for item in group.split(",") {
            let mut i: Vec<i32> = Vec::new();
            for x in item.split("-") {
                i.push(x.to_string().parse::<i32>().unwrap());
            }
            items.push(i);
        }
        groups.push(items);
    }
    groups
}


/// Uses the needle in a haystack algorithm to compare items in a vector. Returns the number of true results.
fn compare_ranges(arr: &Vec<Vec<Vec<i32>>>, strict: bool) -> usize {
    let mut ranges = Vec::new();
    for items in arr {
        let mut contains = Vec::new();
        let left: Vec<i32> = aoc_lib::generate_vec(items[0][0], items[0][1]);
        let right: Vec<i32> = aoc_lib::generate_vec(items[1][0],items[1][1]);
        
        if aoc_lib::find_sub(&right, &left, strict) || 
            aoc_lib::find_sub(&left, &right, strict) 
        {
            contains.push(true);
        }

        if contains.len() >= 1 {
            ranges.push(contains)
        }

        
    }
    return ranges.len();
}

fn main() {
    let items = parse_file("inputs/day4/list.txt");
    let part1 = compare_ranges(&items, true);
    let part2 = compare_ranges(&items, false);
    println!("{:?}", part1);
    println!("{:?}", part2);
}
