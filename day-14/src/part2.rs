use crate::parse_input;
use std::path::Path;

const W: i32 = 101;
const H: i32 = 103;

pub fn process<P>(filename: P) -> u32 where P: AsRef<Path>  {
    let robots = parse_input(filename);
    let mut min_sf = std::i32::MAX;
    let mut best_iter = 0;
    for i in 0..H*W {
        let mut result:Vec<(i32,i32)>  = vec![];
        for x in robots.iter() {
            let (px, py, vx, vy) = (x[0], x[1], x[2], x[3]);
            result.push(((px+vx*i).rem_euclid(W), (py+vy*i).rem_euclid(H)));
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

        let sf = quad[0]*quad[1]*quad[2]*quad[3];
        if sf<min_sf {
            min_sf = sf;
            best_iter = i;
        }
    }
    let mut grid = vec![vec!['.';W as usize];H as usize];
    let mut result:Vec<(i32,i32)>  = vec![];
    for x in robots.iter() {
        let (px, py, vx, vy) = (x[0], x[1], x[2], x[3]);
        result.push(((px+vx*best_iter).rem_euclid(W), (py+vy*best_iter).rem_euclid(H)));
    }
    for (px, py) in result {
        grid[py as usize][px as usize] = '@';
    }
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    best_iter as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(0, process("src/test.txt"));
    }
}