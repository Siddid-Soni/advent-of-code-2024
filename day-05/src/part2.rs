use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn process<P>(filename: P) -> i32 where P: AsRef<Path>  {
    let mut map: HashMap<i32,Vec<i32>> = HashMap::new();
    let mut vec: Vec<Vec<i32>> = Vec::new();
    
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let lst: Vec<String> = lines.map(|line| line.unwrap()).collect();
    
    
    let idx = lst.iter().position(|x| x=="").unwrap();
    let v1 = lst.get(..idx).unwrap();
    let v2 = lst.get(idx+1..).unwrap();

    for i in v1 {
        let nums = i.split("|").collect::<Vec<_>>();
        let key = nums[0].parse::<i32>().unwrap();
        let value = nums[1].parse::<i32>().unwrap();
        let values: Vec<i32> = vec![value];
        map.entry(key).and_modify(|x| x.push(value)).or_insert(values);
    }

    for i in v2 {
        let nums = i.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        vec.push(nums);
    }

    fn compare(a: i32, b: i32, map: &HashMap<i32,Vec<i32>>) -> std::cmp::Ordering {
        let b_in_a = map.get(&a).is_some_and(|a| a.contains(&b));
        let a_in_b = map.get(&b).is_some_and(|b| b.contains(&a));
        a_in_b.cmp(&b_in_a)
    }

    let mut sum = 0;
    'outer: for mut line in vec.clone() {
        for j in 1..line.len(){
            if map.contains_key(&line[j]) {
                for k in 0..j {
                    if map.get(&line[j]).unwrap().contains(&line[k]) {
                        line.sort_unstable_by(|&a, &b| compare(a, b, &map));
                        sum += line[line.len()/2];
                        continue 'outer;
                    }
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(123, process("src/test.txt"));
    }
}