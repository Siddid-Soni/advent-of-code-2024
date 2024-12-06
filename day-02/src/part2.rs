use crate::{parse_input, is_safe};
use std::path::Path;

pub fn process<P>(filename: P) -> u32 where P: AsRef<Path>  {
    let lst = parse_input(filename);
    let mut ans = 0;
    for row in lst {
        let mut any_ok = false;

        let mut consider = |x: usize| {
            let mut b = row.clone();
            b.remove(x);
            if is_safe(b) {
                any_ok = true;
            }
        };

        /* remove any element and check is_safe, if it is still safe
            -> the element was the problem, we got the solution
            -> or it was already in safe state 
        either way we need to increment in ans */

        consider(0);
        for i in 0..row.len()-1 {
            let diff = row[i+1] - row[i];
            if diff.abs() < 1 || diff.abs() > 3 {
                consider(i);
                consider(i+1);
                break;
            }

            if i+2 < row.len() {
                let diff2 = row[i+2] - row[i+1];
                if (diff>0) != (diff2>0) { //different signs
                    consider(i);
                    consider(i+1);
                    consider(i+2);
                    break;
                }
            }
        }
        if any_ok {
            ans += 1;
        }
    } 
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(4, process("src/test.txt"));
    }
}