use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub fn process<P>(filename: P) -> u64 where P: AsRef<Path>  {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let binding = lines.map(|line| line.unwrap()).collect::<Vec<_>>();
    let lines = binding.split(|x| x=="").collect::<Vec<_>>();
    let re = Regex::new(r"\d+").unwrap();
    let (a, b, p) = (0_usize, 1_usize, 2_usize);
    let (x, y) = (0_usize, 1_usize);

    let mut total = 0;
    for block in &lines {
        let mut nums = block.iter().map(|x| re.find_iter(x).map(|x| x.as_str().parse::<f64>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
        nums[p][x] += 10000000000000.0;
        nums[p][y] += 10000000000000.0;
        let ca = (nums[p][x] * nums[b][y] - nums[p][y] * nums[b][x]) / (nums[a][x] * nums[b][y] - nums[a][y] * nums[b][x]);
        let cb = (nums[p][x] - nums[a][x] * ca) / nums[b][x];
        if (ca%1.0) == 0.0 && (cb%1.0) == 0.0 {
            total += (ca * 3.0 + cb) as u64;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(0, process("src/test.txt"));
    }
}