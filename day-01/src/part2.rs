use crate::parse_input;
use std::path::Path;
use std::collections::HashMap;

pub fn process<P>(filename: P) -> i32 where P: AsRef<Path>  {
    let (v1, v2) = parse_input(filename);
    let mut map = HashMap::new();
    for x in v2.iter() {
        map.entry(x).and_modify(|c| *c += 1).or_insert(1);
    }
    v1.iter().map(|x| x * map.get(x).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(31, process("src/test.txt"));
    }
}