use crate::parse_input;
use std::path::Path;

pub fn process<P>(filename: P) -> u32 where P: AsRef<Path>  {
    let grid = parse_input(filename);
    let mut sum = 0;
    fn dfs(r: i32, c: i32, next: i32, grid: &Vec<Vec<i32>>, visited: &mut Vec<bool>) -> u32 {
        if !(0<=r && r<grid.len() as i32) || !(0<=c && c<grid[0].len() as i32) || grid[r as usize][c as usize] != next {return 0}
        if next == 9 && !visited[(r * grid[0].len() as i32 + c) as usize] {
            visited[(r * grid[0].len() as i32 + c) as usize] = true;
            return 1;
        }
        
        dfs(r+1, c, next+1, &grid, visited) + dfs(r-1, c, next+1, &grid, visited) 
                + dfs(r, c+1, next+1, &grid, visited) + dfs(r, c-1, next+1, &grid, visited)
    }

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 {
                let mut visited = vec![false; grid.len() * grid[0].len()];
                sum+=dfs(r as i32, c as i32, 0, &grid, &mut visited);
            }
        }
    }
    sum

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(36, process("src/test.txt"));
    }
}