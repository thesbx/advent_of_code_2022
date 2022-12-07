use std::fs;

fn parse_file(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("Couldn't read file");
    let mut tree = Vec::new();
    for content in file.split("\n") {
        for string in content.split_whitespace() {
            tree.push(string.to_string());
        }
    }
    tree
}

fn process_commands(input: &Vec<String>) {
    let mut current_dir = String::new(); 
    let mut parent_dir = &String::new();

    for line in input {
       if line.contains("$ cd ") {
           let dir = &line[5..];
           if dir == ".." {
               current_dir = parent_dir.to_string();
           } else {
               parent_dir = &current_dir.to_string();
               current_dir = current_dir.to_owned() + &String::from("/") + dir;
           }
               
       }
    }
    println!("{}", current_dir);
}

fn main() {
    let commands = parse_file("./test.txt");
    process_commands(&commands);
    println!("{:?}", commands);
}
