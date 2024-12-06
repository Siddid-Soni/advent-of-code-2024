pub mod part1;
pub mod part2;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input<P>(filename: P) -> (Vec<i32>, Vec<i32>) where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| {
        let res = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        (res[0], res[1])
    }).unzip()
}