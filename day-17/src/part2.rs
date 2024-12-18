pub fn process() -> u64 {
    let program: Vec<u64> = vec![2,4,1,1,7,5,1,5,0,3,4,3,5,5,3,0];
    
    fn find(program: Vec<u64>, ans: u64) -> Option<u64> {
        if program.len() == 0 { return Some(ans); }
        for mut b in 0..8 {
            let a = (ans << 3) + b;
            b ^= 1;
            let c = a >> b;
            b ^= 5;
            b ^= c;

            if b % 8 == program[program.len() - 1] {
                let sub = find(program[..program.len()-1].to_vec(), a);
                if sub.is_none() {continue;}
                return sub;
            }
        }
        None
    }
    
    find(program, 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(0, process());
    }
}