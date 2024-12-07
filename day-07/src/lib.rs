pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input<P>(filename: P) -> (Vec<u64>, Vec<Vec<u64>>) where P: AsRef<Path> {
    let mut v1: Vec<u64> = vec![];
    let mut v2: Vec<Vec<u64>> = vec![];
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.for_each(|line| {
        let line = line.unwrap();
        let res: Vec<&str> = line.split(": ").collect();
        v1.push(res[0].parse::<u64>().unwrap());
        v2.push(res[1].trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>());
    });
    (v1,v2)
}