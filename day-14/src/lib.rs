pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub fn parse_input<P>(filename: P) -> Vec<Vec<i32>> where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let re = Regex::new(r"-?\d+").unwrap();
    lines.map(|x| re.find_iter(&x.unwrap()).map(|x| x.as_str().parse().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()
}