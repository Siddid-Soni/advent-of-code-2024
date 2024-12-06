use std::path::Path;
use regex::Regex;

pub fn process<P>(filename: P) -> i32 where P: AsRef<Path>  {
    let input = std::fs::read_to_string(filename).unwrap();
    let re = Regex::new(r"don't\(\)|do\(\)|mul\(\d+,\d+\)").unwrap();

    let mut ans = 0;
    let mut flag = true;

    for cap in re.find_iter(&input) {
        if cap.as_str() == "do()" {
            flag = true;
        } else if cap.as_str() == "don't()" {
            flag = false;
        } else {
            if flag {
                let s = cap.as_str().split(",").collect::<Vec<_>>();
                ans += s[0][4..].parse::<i32>().unwrap()*s[1][..s[1].len()-1].parse::<i32>().unwrap();
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(42, process("src/test.txt"));
    }
}