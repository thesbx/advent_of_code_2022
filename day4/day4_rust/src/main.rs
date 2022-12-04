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

fn is_sub<T: PartialEq>(haystack: &[T], needle: &[T], strict: bool) -> bool {
    if strict {
        return needle.iter().all(|x| haystack.contains(&x));
    } else {
        return needle.iter().any(|x| haystack.contains(&x));
    }
    
}

fn compare_ranges(arr: &Vec<Vec<Vec<i32>>>, strict: bool) -> usize {
    let mut ranges = Vec::new();
    for items in arr {
        let mut contains = Vec::new();
        let left: Vec<i32> = (items[0][0]..items[0][1] + 1).collect();
        let right: Vec<i32> = (items[1][0]..items[1][1] + 1).collect();
        
        if is_sub(&right, &left, strict) {
            contains.push(true);
        }

        if is_sub(&left, &right, strict) {
            contains.push(true);
        }

        if contains.len() >= 1 {
            ranges.push(contains)
        }

        
    }
    return ranges.len();
}

fn main() {
    let items = parse_file("../list.txt");
    let part1 = compare_ranges(&items, true);
    let part2 = compare_ranges(&items, false);
    println!("{:?}", part1);
    println!("{:?}", part2);
}
