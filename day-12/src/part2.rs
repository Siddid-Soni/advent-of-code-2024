use crate::parse_input;
use std::path::Path;

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn process<P>(filename: P) -> i32 where P: AsRef<Path>  {
    let grid = parse_input(filename);
    let mut visited = vec![false; grid.len() * grid[0].len()];

    fn dfs(r: i32, c:i32, crop: char, area: &mut i32, perim: &mut i32, grid: &Vec<Vec<char>>, visited: &mut Vec<bool>) {
        visited[(r * grid[0].len() as i32 +c) as usize] = true;
        *area+=1;

        let good = |dir: (i32, i32)| {
            let r2 = r + dir.0;
            let c2 = c + dir.1;
            r2 >= 0 && r2 < grid.len() as i32 && c2 >= 0 && c2 < grid[0].len() as i32 && grid[r2 as usize][c2 as usize] == grid[r as usize][c as usize]
        };

        for i in 0..4 {
            let dir = DIRS[i];
            let dir2 = DIRS[(i+1)%4];
            if !good(dir) && !good(dir2) {
                *perim+=1;
            }
            if good(dir) && good(dir2) && !good((dir.0+dir2.0, dir.1+dir2.1)) {
                *perim+=1;
            }
        }

        for dir in DIRS {
            let r2 = r + dir.0;
            let c2 = c + dir.1;

            if good(dir) && !visited[(r2 * grid[0].len() as i32 + c2) as usize] {
                dfs(r2, c2, crop, area, perim, grid, visited);
            }
        }
    }

    let mut price = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if visited[(r * grid[0].len() + c) as usize] {continue;}
            let mut area = 0;
            let mut perim = 0;
            dfs(r as i32, c as i32, grid[r][c], &mut area, &mut perim, &grid, &mut visited);
            price += perim*area;
        }
    }
    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(1206, process("src/test.txt"));
    }
}