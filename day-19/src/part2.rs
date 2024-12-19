use crate::parse_input;
use std::{cmp::min, path::Path, collections::HashMap};

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let (patterns, designs) = parse_input(filename);
    let max_len = patterns.iter().map(|x| x.len()).collect::<Vec<_>>().iter().copied().max().unwrap();

    let mut cache: HashMap<Vec<char>, usize> = HashMap::new();

    fn can_obtain(patterns: &Vec<Vec<char>>, design: &Vec<char>, max_len: usize, cache: &mut HashMap<Vec<char>, usize>) -> usize {
        if design.is_empty() {return 1}
        let mut count = 0;
        if cache.contains_key(design) {return cache[design];}
        for i in 0..min(design.len(), max_len) + 1 {
            if patterns.contains(&design[..i].to_vec()) { 
                count+=can_obtain(patterns, &design[i..].to_vec(), max_len, cache);
                cache.entry(design.clone()).and_modify(|c| *c=count).or_insert(count);
                
            }
        }
        count
    }

    designs.iter().map(|x| can_obtain(&patterns, x, max_len, &mut cache)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(16, process("src/test.txt"));
    }
}