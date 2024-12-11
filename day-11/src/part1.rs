use crate::parse_input;
use std::path::Path;

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let mut stones = parse_input(filename);
    for _ in 0..25 {
        let mut output: Vec<u64> = Vec::with_capacity(stones.len()*2);
        for j in 0..stones.len() {
            if stones[j] == 0 {
                output.push(1);
            } else if (stones[j].ilog10()+1)%2==0 { 
                output.push(stones[j]/10_u64.pow((stones[j].ilog10()+1)/2));
                output.push(stones[j]%10_u64.pow((stones[j].ilog10()+1)/2));
            } else {
                output.push(stones[j] * 2024);
            }
        }
        stones = output;
    }
    stones.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(55312, process("src/test.txt"));
    }
}