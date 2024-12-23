use crate::parse_input;
use std::{collections::HashSet, path::Path};

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let graph = parse_input(filename);
    
    let mut sets = HashSet::new();
    for x in graph.keys() {
        for y in &graph[x] {
            for z in &graph[y] {
                if x!=z && graph[z].contains(x) {
                    let mut pair = [x,y,z];
                    pair.sort_unstable();
                    sets.insert(pair);
                }
            }
        }
    }
    sets.iter().filter(|&pair| pair.iter().any(|&x| x.starts_with("t"))).collect::<Vec<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(7, process("src/test.txt"));
    }
}