use crate::parse_input;
use std::path::Path;

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let seq = parse_input(filename);
    let mut sum = 0;
    let mut aloc: Vec<u32> = Vec::with_capacity(9*seq.len());

    let mut j = 0;
    for (i, num) in seq.iter().enumerate() {
        if i%2==0 {
            aloc.extend(vec![j + b'0' as u32;(*num - b'0') as usize]);
            j+=1;
        } else {
            aloc.extend(vec![46;(*num - b'0') as usize]);
        }
    }

    j = (aloc.len() - 1) as u32;
    for i in 0..aloc.len() {
        if i==j as usize {break}
        if aloc[i]==b'.' as u32 && aloc[j as usize]!=b'.' as u32 {
            aloc.swap(i, j as usize);
            j-=1;
        } else if aloc[i]==b'.' as u32 && aloc[j as usize]==b'.' as u32 {
            while aloc[j as usize] == b'.' as u32 {
                j-=1;
            }
            aloc.swap(i, j as usize);
            j-=1;
        }
    }

    for (i, num) in aloc.iter().enumerate() {
        if i == j as usize+1 {
            break;
        }
        sum += i * (num - b'0' as u32) as usize;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(1928, process("src/test.txt"));
    }
}