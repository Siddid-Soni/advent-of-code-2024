pub mod part1;
pub mod part2;
pub mod part2_2;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input<P>(filename: P) -> Vec<u64> where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|x| x.unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<u64>>()).flatten().collect::<Vec<u64>>()
}