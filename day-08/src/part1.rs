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
    // let mut count = 0;
    for arr in map.values() {
        for i in 0..arr.len() {
            for j in i+1..arr.len() {
                let (r1, c1) = arr[i];
                let (r2, c2) = arr[j];
                let (ar1, ac1) = (2*r1-r2, 2*c1-c2);
                let (ar2, ac2) = (2*r2-r1, 2*c2-c1);
                if 0<=ar1 && ar1<grid.len() as i32 && 0<=ac1 && ac1 < grid[0].len() as i32 {
                    set.insert((ar1,ac1));
                }
                if 0<=ar2 && ar2<grid.len() as i32 && 0<=ac2 && ac2 < grid[0].len() as i32 {
                    set.insert((ar2,ac2));
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
        assert_eq!(14, process("src/test.txt"));
    }
}