pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input(filename: impl AsRef<Path>) -> Vec<(usize,usize)>  {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| {let bind = line.unwrap(); 
        let x: Vec<_> = bind.split(",").collect(); 
        (x[0].parse::<usize>().unwrap(),x[1].parse::<usize>().unwrap())}).collect()
}