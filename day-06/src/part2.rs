use crate::parse_input;
use std::path::Path;

#[allow(non_snake_case)]
pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let dirs: [(i32, i32); 4] = [(-1,0),(0,1),(1,0),(0,-1)];
    let mut grid = parse_input(filename);
    let H = grid.len();
    let W = grid[0].len();
    
    let mut start = (0,0);
    for row in 0..H {
        for col in 0..W {
            if grid[row][col] == 94 {
                start = (row, col);
            }
        }
    }
    
    let solve = |grid: &Vec<Vec<u8>>| {
        let mut dir = 0;
        let mut me = start;

        let mut vis = vec![false;H*W*4];
        // let mut turns = 0;
        loop {
            // turns+=1;
            let hash = (me.0 * W + me.1) * 4 + dir;
            if vis[hash] {return true}
            vis[hash] = true;
            // if turns == (H*W) {
            //     return true;
            // }

            let r2 = me.0 as i32 + dirs[dir].0;
            let c2 = me.1 as i32 + dirs[dir].1;
            
            if !(0<=r2 && r2<H as i32 && 0<=c2 && c2<W as i32) {
                return false;
            }
            
            if grid[r2 as usize][c2 as usize] == 35 {
                dir = (dir+1)%4;
            } else {
                me = (r2 as usize,c2 as usize);
            }
        }
    };

    let write_path = |grid: &mut Vec<Vec<u8>>| {
        let mut dir = 0;
        let mut me = start;

        loop {
            grid[me.0][me.1] = 88;

            let r2 = me.0 as i32 + dirs[dir].0;
            let c2 = me.1 as i32 + dirs[dir].1;
            
            if !(0<=r2 && r2<H as i32 && 0<=c2 && c2<W as i32) {
                break;
            }
            
            if grid[r2 as usize][c2 as usize] == 35 {
                dir = (dir+1)%4;
            } else {
                me = (r2 as usize,c2 as usize);
            }
        }
    };

    write_path(&mut grid);
    let mut count = 0;
    for row in 0..H {
        for col in 0..W {
            if grid[row][col]!=88 {continue}
            grid[row][col] = 35;
            if solve(&grid) {
                count+=1;
            }
            grid[row][col] = 46;
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