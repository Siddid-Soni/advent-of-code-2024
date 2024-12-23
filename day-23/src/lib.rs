pub mod part1;
pub mod part2;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input<P>(filename: P) -> HashMap<String, HashSet<String>> where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut conn = HashMap::new();
    lines.for_each(|line| {
        let bind = line.unwrap();
        let xy = bind.split('-').collect::<Vec<_>>();
        let x = xy[0].to_string();
        let y = xy[1].to_string();
        conn.entry(x.clone()).or_insert(HashSet::new()).insert(y.clone());
        conn.entry(y).or_insert(HashSet::new()).insert(x);
    });
    conn
}