use crate::parse_input;
use std::path::Path;

pub fn process<P>(filename: P) -> u32 where P: AsRef<Path>  {
    let grid = parse_input(filename);
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 88 { continue }
            for dr in [-1,0,1] {
                for dc in [-1,0,1] {
                    if dr == 0 && dc == 0 {continue}
                    if !((i as i32 + 3*dr >= 0 && i as i32 + 3*dr < grid.len() as i32) && 
                    (j as i32 + 3*dc >= 0 && j as i32 + 3*dc < grid[0].len() as i32)) {continue}
                    
                    if grid[(i as i32 + dr) as usize][(j as i32 + dc) as usize] == 77 &&
                    grid[(i as i32 + 2 * dr) as usize][(j as i32 + 2 * dc) as usize] == 65 && 
                    grid[(i as i32 + 3 * dr) as usize][(j as i32 + 3 * dc) as usize]== 83 {
                        count+=1;
                    }
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
        assert_eq!(18, process("src/test.txt"));
    }
}