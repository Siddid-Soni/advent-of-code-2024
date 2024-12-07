use crate::parse_input;
use std::path::Path;

fn can_obtain(target:u64, nums: &Vec<u64>) -> bool {
    if nums.len() == 1 {return target == nums[0]}
    if target % nums[nums.len()-1] == 0 && can_obtain(target / nums[nums.len()-1],&nums[..nums.len()-1].to_vec()) {return true}
    if target > nums[nums.len()-1] && can_obtain(target-nums[nums.len()-1], &nums[..nums.len()-1].to_owned()) {return true}
    if target.ilog10()>nums[nums.len()-1].ilog10() 
    && target%10_u64.pow(nums[nums.len()-1].ilog10()+1) == nums[nums.len()-1]
    && can_obtain(target / 10_u64.pow(nums[nums.len()-1].ilog10()+1), &nums[..nums.len()-1].to_owned()) {return true}
    return false;
}

pub fn process<P>(filename: P) -> u64 where P: AsRef<Path>  {
    let (v1, v2) = parse_input(filename);
    let mut ans = vec![];
    for i in 0..v1.len() {
        if can_obtain(v1[i], &v2[i]) {
            ans.push(v1[i]);
        }
    }
    ans.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(11387, process("src/test.txt"));
    }
}