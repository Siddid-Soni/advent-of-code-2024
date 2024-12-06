pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input<P>(filename: P) -> Vec<Vec<u8>> where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap().as_bytes().to_vec()).collect::<Vec<_>>()
}