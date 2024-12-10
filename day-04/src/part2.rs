use crate::parse_input;
use std::path::Path;

pub fn process<P>(filename: P) -> u32 where P: AsRef<Path>  {
    let grid = parse_input(filename);
    let mut count = 0;
    for i in 1..grid.len()-1 {
        for j in 1..grid[0].len()-1 {
            if grid[i][j] != b'A' { continue };
            let corners = [grid[i-1][j-1], grid[i-1][j+1], grid[i+1][j+1], grid[i+1][j-1]];
            if [[b'M',b'M',b'S',b'S'],[b'M',b'S',b'S',b'M'],[b'S',b'S',b'M',b'M'],[b'S',b'M',b'M',b'S']].contains(&corners) { count+=1 }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(9, process("src/test.txt"));
    }
}