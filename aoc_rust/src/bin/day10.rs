use anyhow::Result;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/day10.test")?;
    let lines = file.split("\n").collect::<Vec<_>>();
    lines.iter().map(|x| {
        x.split(" ").collect::<Vec<_>>()
    });

    for line in lines {
        let single = line.split(" ").collect::<Vec<_>>();
    }
    return Ok(())
}

