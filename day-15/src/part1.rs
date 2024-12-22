use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let lines = lines.map(|line| line.unwrap()).collect::<Vec<_>>();
    let lines = lines.split(|x| x=="").collect::<Vec<_>>();
    let mut grid = lines[0].iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let moves = lines[1].join("").chars().collect::<Vec<_>>();
    
    let (mut r, mut c) = (0_i32, 0_i32);
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                (r, c) = (i as i32, j as i32);
                break 'outer;
            }
        }
    }

    for &mov in moves.iter() {
        let dr = if '^'==mov { -1 } else if 'v'==mov { 1 } else { 0 };
        let dc = if '<'==mov { -1 } else if '>'==mov { 1 } else { 0 };
        let mut targets = vec![(r,c)];
        let (mut cr, mut cc) = (r, c);
        let mut go = true;
        loop {
            cr += dr;
            cc += dc;
            let ch = grid[cr as usize][cc as usize];
            if ch == '#' {
                go = false;
                break;
            }
            if ch == 'O' { targets.push((cr, cc)); }
            if ch == '.' { break; }
        }
        if !go { continue; }
        grid[r as usize][c as usize] = '.';
        grid[(r + dr) as usize][(c + dc) as usize] = '@';
        for (br, bc) in &targets[1..] {
            grid[(*br + dr) as usize][(*bc + dc) as usize] = 'O';
        }
        r+=dr;
        c+=dc;
    }
    
    // println!("{:#?}", grid.iter().map(|x| x.iter().collect::<String>()).collect::<Vec<_>>());
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
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
        assert_eq!(10092, process("src/test.txt"));
    }
}