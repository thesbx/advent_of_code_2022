use anyhow::Result;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/day10.test")?;
    let lines = file.split("\n").collect::<Vec<_>>();
    let commands = lines
    
    for line in lines {

        if line.split_once(" ").is_some() {
            let (cmd, n) = line.split_once(" ").unwrap();
            println!("{:?} --- {:?}", cmd, n.parse::<i32>().unwrap());
        }
    }
    
    return Ok(())
}

