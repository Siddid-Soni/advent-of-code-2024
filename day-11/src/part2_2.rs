use crate::parse_input;
use std::{collections::HashMap, path::Path};

pub fn process<P>(filename: P) -> u64 where P: AsRef<Path>  {
    let stones = parse_input(filename);
    let mut v: HashMap<u64,u64> = HashMap::new();
    for i in 0..stones.len() {
        *v.entry(stones[i]).or_insert(0) += 1;
    }
    for _ in 0..75 {
        let mut new_v: HashMap<u64,u64> = HashMap::new();

        for (&x, &count) in v.iter() {
            if x == 0 {
                *new_v.entry(1).or_insert(0)+=count;
            } else if (x.ilog10()+1)%2==0 {
                *new_v.entry(x/10_u64.pow((x.ilog10()+1)/2)).or_insert(0)+=count;
                *new_v.entry(x%10_u64.pow((x.ilog10()+1)/2)).or_insert(0)+=count;
            } else {
                *new_v.entry(x * 2024).or_insert(0)+=count;
            }
        }
        v = new_v;
    }
    v.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(55312, process("src/test.txt"));
    }
}