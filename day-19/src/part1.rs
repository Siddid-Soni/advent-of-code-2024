use crate::parse_input;
use std::{cmp::min, path::Path, collections::HashMap};

pub fn process<P>(filename: P) -> u32 where P: AsRef<Path>  {
    let (patterns, designs) = parse_input(filename);
    let max_len = patterns.iter().map(|x| x.len()).collect::<Vec<_>>().iter().copied().max().unwrap();

    let mut cache: HashMap<Vec<char>, bool> = HashMap::new();

    fn can_obtain(patterns: &Vec<Vec<char>>, design: &Vec<char>, max_len: usize, cache: &mut HashMap<Vec<char>, bool>) -> bool {
        if design.is_empty() {return true}
        if cache.contains_key(design) {return cache[design];}
        for i in 0..min(design.len(), max_len) + 1 {
            if patterns.contains(&design[..i].to_vec()) && can_obtain(patterns, &design[i..].to_vec(), max_len, cache) {
                cache.insert(design.clone(), true);
                return true;
            }
        }
        cache.insert(design.clone(), false);
        false
    }

    designs.iter().map(|x| if can_obtain(&patterns, x, max_len, &mut cache) {1} else {0}).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(6, process("src/test.txt"));
    }
}