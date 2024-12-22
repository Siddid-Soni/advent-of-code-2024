use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let lines = lines.map(|line| line.unwrap()).collect::<Vec<_>>();
    let lines = lines.split(|x| x=="").collect::<Vec<_>>();
    let grid = lines[0].iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut grid_new: Vec<Vec<char>> = Vec::with_capacity(grid.len()*grid[0].len()*2);

    for i in 0..grid.len() {
        let mut temp = Vec::with_capacity(grid[0].len()*2);
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                temp.push('@');
                temp.push('.');
            } else if grid[i][j] == '#' {
                temp.push('#');
                temp.push('#');
            } else if grid[i][j] == 'O' {
                temp.push('[');
                temp.push(']');
            } else if grid[i][j] == '.' {
                temp.push('.');
                temp.push('.');
            }
        }
        grid_new.push(temp);
    }
    let moves = lines[1].join("").chars().collect::<Vec<_>>();
    
    let (mut r, mut c) = (0_i32, 0_i32);
    'outer: for i in 0..grid_new.len() {
        for j in 0..grid_new[0].len() {
            if grid_new[i][j] == '@' {
                (r, c) = (i as i32, j as i32);
                break 'outer;
            }
        }
    }

    for &mov in moves.iter() {
        let dr = if '^'==mov { -1 } else if 'v'==mov { 1 } else { 0 };
        let dc = if '<'==mov { -1 } else if '>'==mov { 1 } else { 0 };
        let mut targets = vec![(r,c)];
        let mut go = true;

        let mut i = 0;
        while i < targets.len() {
            let (cr, cc) = targets[i];
            let nr = cr + dr;
            let nc = cc + dc;
            if targets.contains(&(nr, nc)) {i+=1;continue;}
            let ch = grid_new[nr as usize][nc as usize];
            if ch == '#' {
                go = false;
                break;
            }
            if ch == '[' {
                targets.push((nr, nc));
                targets.push((nr, nc + 1));
            }
            if ch == ']' {
                targets.push((nr, nc));
                targets.push((nr, nc - 1));
            }
            i+=1;
        }
        if !go { continue; }
        let grid_copy = grid_new.clone();
        grid_new[r as usize][c as usize] = '.';
        grid_new[(r + dr) as usize][(c + dc) as usize] = '@';
        
        for (br, bc) in targets[1..].iter() {
            grid_new[*br as usize][*bc as usize] = '.';
        }
        for (br, bc) in targets[1..].iter() {
            grid_new[(br+dr) as usize][(bc+dc) as usize] = grid_copy[*br as usize][*bc as usize];
        }
        r+=dr;
        c+=dc;
    }
    
    let mut total = 0;
    for i in 0..grid_new.len() {
        for j in 0..grid_new[0].len() {
            if grid_new[i][j] == '[' {
                total += 100*i + j;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(9021, process("src/test.txt"));
    }
}