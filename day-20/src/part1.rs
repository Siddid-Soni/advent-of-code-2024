use crate::parse_input;
use std::path::Path;

pub fn process<P>(filename: P) -> u32 where P: AsRef<Path>  {
    let grid = parse_input(filename);
    let (mut r, mut c) = (0_i32, 0_i32);
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                (r, c) = (i as i32, j as i32);
                break 'outer;
            }
        }
    }
    
    let mut dist: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
    dist[r as usize][c as usize] = 0;

    while grid[r as usize][c as usize] != 'E' {
        for (nr, nc) in [(r, c+1), (r+1, c), (r, c-1), (r-1, c)] {
            if nr<0 || nr >= grid.len() as i32 || nc<0 || nc >= grid[0].len() as i32 {continue;}
            if grid[nr as usize][nc as usize] == '#' {continue;}
            if dist[nr as usize][nc as usize] != -1 {continue;}
            dist[nr as usize][nc as usize] = dist[r as usize][c as usize] + 1;
            r = nr;
            c = nc;
        }
    }

    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '#' {continue;}

            for (nr, nc) in [(r as i32 + 2, c as i32), (r as i32+ 1, c as i32 + 1), (r as i32, c as i32 +2), (r as i32 - 1, c as i32 + 1)] {
                if nr<0 || nr >= grid.len() as i32 || nc<0 || nc >= grid[0].len() as i32 {continue;}
                if grid[nr as usize][nc as usize] == '#' {continue;}
                if dist[r][c].abs_diff(dist[nr as usize][nc as usize]) >= 102 {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(0, process("src/test.txt"));
    }
}