use crate::parse_input;
use std::path::Path;
use std::collections::HashSet;

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let mut grid = parse_input(filename);
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut r = 0;
    let mut c = 0;
    'outer: for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 94 {
                (r,c) = (i,j);
                break 'outer;
            }
        }
    }
    
    let loops = |grid: Vec<Vec<u8>>, mut r, mut c| {
        let mut seen: HashSet<(usize, usize, i32, i32)> = HashSet::new();
        let mut dr: i32 = -1;
        let mut dc: i32 = 0;
    
        loop {
            seen.insert((r, c, dr, dc));
            if r as i32 + dr < 0 || r as i32 + dr >= rows as i32 || c as i32 + dc < 0 || c as i32 + dc >= cols as i32 { return false }
            
            if grid[(r as i32 + dr) as usize][(c as i32 + dc) as usize] == 35 {
                std::mem::swap(&mut dr, &mut dc);
                dc *= -1;
            } else {
                r = (r as i32 + dr) as usize;
                c = (c as i32 + dc) as usize;
            }
            if seen.contains(&(r,c,dr,dc)) { return true }
        }
    };

    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j]!=46 {continue}
            grid[i][j] = 35;
            if loops(grid.clone(), r.clone(), c.clone()) {
                count+=1;
            }
            grid[i][j] = 46;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(6, process("src/test.txt"));
    }
}