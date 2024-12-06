use std::path::Path;
use regex::Regex;

pub fn process<P>(filename: P) -> i32 where P: AsRef<Path>  {
    let input = std::fs::read_to_string(filename).unwrap();
    Regex::new(r"mul\(\d+,\d+\)").unwrap().find_iter(&input)
            .map(|x| x.as_str().split(",").collect::<Vec<_>>())
            .map(|s| s[0][4..].parse::<i32>().unwrap()*s[1][..s[1].len()-1].parse::<i32>().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(161, process("src/test.txt"));
    }
}