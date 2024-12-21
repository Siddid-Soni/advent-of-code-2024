use crate::parse_input;
use std::{collections::{HashMap, VecDeque}, path::Path};


fn get_sequence(keypad: &Vec<Vec<char>>) -> HashMap::<(char,char), Vec<String>> {
    let mut pos = HashMap::<char, (usize,usize)>::new();
    for r in 0..keypad.len() {
        for c in 0..keypad[0].len() {
            if keypad[r][c] != '#' { pos.insert(keypad[r][c], (r,c));}
        }
    }
    let mut seqs = HashMap::<(char,char), Vec<String>>::new();
    for x in pos.keys() {
        for y in pos.keys() {
            if x == y {
                seqs.insert((*x,*y), vec!["A".to_owned()]);
                continue;
            }
            let mut possible = vec![];
            let mut q = VecDeque::from(vec![(pos[x], "".to_owned())]);
            let mut optimal = usize::MAX;

            'outer: while let Some(((r,c), path)) = q.pop_front() {
                for (nr, nc, nm) in [(r as i32 - 1, c as i32, "^"), (r as i32 + 1, c as i32, "v"), (r as i32, c as i32 - 1, "<"), (r as i32, c as i32 + 1, ">")] {
                    if nr < 0 || nr >= keypad.len() as i32 || nc < 0 || nc >= keypad[0].len() as i32 { continue; }
                    if keypad[nr as usize][nc as usize] == '#' { continue; }
                    if keypad[nr as usize][nc as usize] == *y {
                        if optimal<path.len() + 1 { break 'outer; }
                        optimal = path.len() + 1;
                        possible.push(path.clone() + nm + "A");
                    } else {
                        q.push_back(((nr as usize, nc as usize), path.clone() + nm));
                    }
                }
            }
            seqs.insert((*x, *y), possible);
        }
    }
    seqs
}

fn solve(moves: Vec<char>, seqs: &HashMap::<(char,char), Vec<String>>) -> Vec<String> {
    let mut moves_from = vec!['A'];
    moves_from.extend(moves.iter());
    let options = Vec::from_iter(moves_from.iter().zip(&moves).map(|(&x, &y)| seqs[&(x,y)].clone()));
    options.into_iter().fold(vec!["".to_owned()], |acc, x| acc.into_iter().flat_map(|a| x.iter().map(move |y| a.clone() + y)).collect::<Vec<String>>())
}

fn compute_length(seq: String, depth: usize, dir_len: &HashMap<(char, char), usize>, dir_seq: &HashMap<(char, char), Vec<String>>, 
    cache: &mut HashMap<(String, usize), usize>) -> usize 
{
    if depth == 1 { return ("A".to_owned() + &seq).chars().zip(seq.chars()).map(|(a, b)| dir_len[&(a, b)]).sum() }
    if cache.contains_key(&(seq.clone(),depth)) { return cache[&(seq, depth)]; }
    let mut length = 0;
    for (x, y) in ("A".to_owned() + &seq).chars().zip(seq.chars()) {
        length += dir_seq[&(x, y)].iter().map(|subseq| compute_length(subseq.clone(), depth - 1, dir_len, dir_seq, cache)).min().unwrap();
    }
    cache.insert((seq, depth), length);
    return length;
}

pub fn process<P>(filename: P) -> usize where P: AsRef<Path>  {
    let door_keypad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['#', '0', 'A']
    ];
        
    let dir_keypad = vec![
        vec!['#', '^', 'A'],
        vec!['<', 'v', '>'],
    ];

    let door_seq = get_sequence(&door_keypad);
    let dir_seq = get_sequence(&dir_keypad);
    let dir_length = HashMap::<(char, char), usize>::from_iter(dir_seq.iter().map(|((x, y), v)| ((*x, *y), v[0].len())));
    let mut cache = HashMap::<(String, usize), usize>::new();

    let mut total = 0;
    for line in parse_input(filename) {
        let inputs = solve(line.clone(), &door_seq);
        let length = inputs.iter().map(|x| compute_length(x.clone(), 25, &dir_length, &dir_seq, &mut cache)).min().unwrap();
        total += length * String::from_iter(&line[..line.len()-1]).parse::<usize>().unwrap();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(0, process("src/test.txt"));
    }
}