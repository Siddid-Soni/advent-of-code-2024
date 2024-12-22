use crate::parse_input;
use std::{collections::{HashMap, HashSet}, path::Path};

fn secret_evolve(num: usize) -> usize {
    let mut num = ((num << 6) ^ num) % 16777216;
    num = ((num >> 5) ^ num) % 16777216;
    num = ((num << 11) ^ num) % 16777216;
    num
} 

pub fn process<P>(filename: P) -> i32 where P: AsRef<Path>  {
    let secret_nums = parse_input(filename);
    let mut seq_to_total = HashMap::new();
    for i in secret_nums {
        let mut result = i;
        let mut buyer = vec![(i % 10) as i32];
        for _ in 0..2000 {
            result = secret_evolve(result);
            buyer.push((result % 10) as i32);
        }

        let mut seen = HashSet::new();
        
        for w in buyer.windows(5) {
            let (a, b, c, d, e) = (w[0], w[1], w[2], w[3], w[4]);
            let seq = (b-a, c-b, d-c, e-d);
            if seen.contains(&seq) { continue; }
            seen.insert(seq);
            if !seq_to_total.contains_key(&seq) { seq_to_total.insert(seq, 0); }
            *seq_to_total.get_mut(&seq).unwrap() += e;
        }
    }
    *seq_to_total.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(0, process("src/test.txt"));
    }
}