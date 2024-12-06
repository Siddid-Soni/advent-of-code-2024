use crate::{parse_input, is_safe};
use std::path::Path;

pub fn process<P>(filename: P) -> i32 where P: AsRef<Path>  {
    let lst = parse_input(filename);
    let mut safe = 0;
    for row in lst {
        if is_safe(row) {
            safe += 1;
        }
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(2, process("src/test.txt"));
    }
}