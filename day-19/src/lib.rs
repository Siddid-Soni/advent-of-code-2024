pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input<P>(filename: P) -> (Vec<Vec<char>>, Vec<Vec<char>>) where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let lines = lines.map(|line| line.unwrap()).collect::<Vec<_>>();
    let lines = lines.split(|x| x == "").collect::<Vec<_>>();
    let (pattern, design) = (lines[0], lines[1]);
    let pattern = &pattern.iter().map(|x| x.split(", ").collect::<Vec<&str>>()).collect::<Vec<_>>()[0];
    (pattern.iter().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>(), design.iter().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>())
}