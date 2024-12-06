pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input<P>(filename: P) -> Vec<Vec<i32>> where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| {
        line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }).collect()
}

fn is_safe(row: Vec<i32>) -> bool {
    let mut direction = [0,0]; 
    for i in 0..row.len()-1 {
        let diff = row[i] - row[i+1];
        if diff.abs()<1 || diff.abs()>3 {
            return false;
        }
        direction[i%2] = diff;
        if (direction[0] > 0 && direction[1] < 0) || (direction[0] < 0 && direction[1] > 0) {
            return false;
        }
    }
    true
}