use crate::parse_input;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let grid = parse_input(filename);
    let mut map: HashMap<char,Vec<(i32,i32)>> = HashMap::new();
    
    for (r, row) in grid.iter().enumerate() {
        for (c, chr) in row.iter().enumerate() {
            if *chr != '.' {
                map.entry(*chr).and_modify(|x| x.push((r as i32,c as i32))).or_insert(vec![(r as i32,c as i32)]);
            }
        }
    }


    let mut set: HashSet<(i32,i32)> = HashSet::new();

    for arr in map.values() {
        for i in 0..arr.len() {
            for j in 0..arr.len() {
                if i == j {continue}
                
                let (r1, c1) = arr[i];
                let (r2, c2) = arr[j];
                let (dr, dc) = (r2-r1, c2-c1); 
                let (mut r, mut c) = (r1,c1);

                while 0<=r && r< grid.len() as i32 && 0<=c && c<grid[0].len() as i32 {
                    set.insert((r,c));
                    r+=dr;
                    c+=dc;
                }
            }
        }
    }

    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(34, process("src/test.txt"));
    }
}