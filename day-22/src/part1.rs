use crate::parse_input;
use std::path::Path;

fn secret_evolve(num: usize) -> usize {
    let mut num = ((num << 6) ^ num) % 16777216;
    num = ((num >> 5) ^ num) % 16777216;
    num = ((num << 11) ^ num) % 16777216;
    num
} 

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let secret_nums = parse_input(filename);
    let mut total = 0;
    for i in secret_nums {
        let mut result = i;
        for _ in 0..2000 {
            result = secret_evolve(result);
        }
        total += result;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(37327623, process("src/test.txt"));
    }
}