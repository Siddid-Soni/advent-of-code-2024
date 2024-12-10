use crate::parse_input;
use std::path::Path;

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let seq = parse_input(filename);
    let mut sum = 0;
    let mut files: Vec<(u32, u32)> = Vec::with_capacity((seq.len()/2)+2);
    let mut blanks: Vec<(u32, u32)> = Vec::with_capacity((seq.len()/2)+2);

    let mut fid = 0;
    let mut pos = 0;
    for (i, num) in seq.iter().enumerate() {
        if i%2==0 {
            files.push((pos, (num - b'0') as u32));
            fid+=1;
        } else {
            if num - b'0' != 0 {
                blanks.push((pos, (num - b'0') as u32));
            }
        }
        pos += (num - b'0') as u32;
    }

    while fid > 0 {
        fid-=1;
        let (pos, size) = files[fid];

        for (i, (start, length)) in blanks.iter().enumerate() {
            if *start>=pos {
                blanks = blanks[..i].to_vec();
                break;
            }
            if size <= *length {
                files[fid] = (*start, size);
                if size == *length {
                    blanks.remove(i);
                } else {
                    blanks[i] = (*start + size, *length - size);
                }
                break;
            }
        }
    }

    for (fid, (pos, size)) in files.iter().enumerate() {
        for x in *pos..pos+size {
            sum += fid * x as usize;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(2858, process("src/test.txt"));
    }
}