pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input<P>(filename: P) -> Vec<Vec<i32>> where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|x| x.unwrap().as_bytes().iter().map(|y| (*y-b'0') as i32).collect()).collect()
}