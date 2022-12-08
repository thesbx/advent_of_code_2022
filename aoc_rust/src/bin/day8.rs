use anyhow::Result;
use std::fs;

#[macro_export]
macro_rules! matrix_coordinates{
    ( $matrix: ident, $row:ident, $col:ident, $x:ident, $y:ident, $visible:ident ) => {
        {
            if $matrix[$x][$y] >= $matrix[$row][$col] {
                $visible = false;
            }
        }
    };
}
#[macro_export]
macro_rules! visibility {
    ( $vis:ident, $counter:ident) => {
        {
            if $vis {
                $counter += 1;
                continue;
            } else {
                $vis = true;
            }
        }
    }
}

fn travers_matrix(filename: &str) -> (u64, u64) {
    let grid = fs::read_to_string(filename).expect("No File")
        .lines()
        .map(|line| {
            return line
                .chars()
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as u32)
                .collect::<Vec<u32>>();
        })
        .collect::<Vec<Vec<u32>>>();
    let mut count_visible = 0_u64;
    for r in 1..grid[0].len() - 1 {
        for c in 1..grid.len() - 1 { 
            let mut visible = true;

            for w in (0..c).rev() {
                matrix_coordinates!(grid, r, c, r, w, visible);
            }

            visibility!(visible, count_visible);

            for e in c + 1..grid[0].len() {
                matrix_coordinates!(grid, r, c, r, e, visible);
            }

            visibility!(visible, count_visible);

            for n in (0..r).rev() {
                matrix_coordinates!(grid, r, c, n, c, visible);
            }

            visibility!(visible, count_visible);

            for s in r + 1..grid.len() {
                matrix_coordinates!(grid, r, c, s, c, visible);
            }
            if visible {
                count_visible += 1;
            }
        }
    }
    let mut vds = vec![];
    for r in 1..grid[0].len() - 1 {
        for c in 1..grid.len() - 1 {
            let mut viewing_distance = [0; 4];

            for w in (0..c).rev() {
                viewing_distance[0] += 1;
                if grid[r][w] >= grid[r][c] {
                    break;
                }
            }

            for e in c + 1..grid[0].len() {
                viewing_distance[1] += 1;
                if grid[r][e] >= grid[r][c] {
                    break;
                }
            }

            for n in (0..r).rev() {
                viewing_distance[2] += 1;
                if grid[n][c] >= grid[r][c] {
                    break;
                }
            }

            for s in r + 1..grid.len() {
                viewing_distance[3] += 1;
                if grid[s][c] >= grid[r][c] {
                    break;
                }
            }

            vds.push(viewing_distance.iter().product());
        }
    }
    (
        count_visible + 2 * (grid[0].len() as u64 + grid.len() as u64 - 2),
        *vds.iter().max().unwrap(),
    )
}
fn main() -> Result<()> {
    let (p1, p2) = travers_matrix("inputs/day8/prod.txt");
    println!("{:?}", p1); 
    println!("{:?}", p2); 

    return Ok(());
}
