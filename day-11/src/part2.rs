use crate::parse_input;
use std::path::Path;
use cached::proc_macro::cached;

#[cached]
fn count(stone: u128, steps: i32) -> u128 {
    if steps == 0 {
        return 1;
    }
    if stone == 0 {
        return count(1, steps-1);
    }
    if (stone.ilog10()+1)%2==0 {
        let half = (stone.ilog10()+1)/2;
        return count(stone/10_u128.pow(half), steps-1) + count(stone%10_u128.pow(half), steps-1);
    }
    count(stone*2024, steps-1)
}

pub fn process<P>(filename: P) -> u128 where P: AsRef<Path>  {
    let stones = parse_input(filename);
    let mut output: Vec<u128> = Vec::with_capacity(stones.len()*2);
    for i in stones {
        output.push(count(i as u128, 75));
    }
    output.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(65601038650482, process("src/test.txt"));
    }
}