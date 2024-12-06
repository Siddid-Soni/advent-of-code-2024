use crate::parse_input;
use std::path::Path;
pub fn process<P>(filename: P) -> u32 where P: AsRef<Path>  {
    let (mut v1, mut v2) = parse_input(filename);
    v1.sort_unstable();
    v2.sort_unstable();

    v1.iter().zip(v2.iter()).map(|(x,y)| (x).abs_diff(*y)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(11, process("src/test.txt"));
    }
}