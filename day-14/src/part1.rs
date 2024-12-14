use crate::parse_input;
use std::path::Path;

const W: i32 = 101;
const H: i32 = 103;

pub fn process<P>(filename: P) -> u32 where P: AsRef<Path>  {
    let robots = parse_input(filename);
    let mut result:Vec<(i32,i32)>  = vec![];
    for x in robots.iter() {
        let (px, py, vx, vy) = (x[0], x[1], x[2], x[3]);
        result.push(((px+vx*100).rem_euclid(W), (py+vy*100).rem_euclid(H)));
    }

    let mut quad = [0;4];
    for (px, py) in result {
        if px == (W-1)/2 || py == (H-1)/2 { continue }
        if px < (W-1)/2 {
            if py < (H-1)/2 { quad[0] += 1; } 
            else { quad[1] += 1; }
        } else {
            if py < (H-1)/2 { quad[2] += 1; } 
            else { quad[3] += 1; }
        }
    }

    quad[0]*quad[1]*quad[2]*quad[3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(12, process("src/test.txt"));
    }
}