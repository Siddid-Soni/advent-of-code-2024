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
    
    let mut total = 0;
    for line in parse_input(filename) {
        let robot1 = solve(line.clone(), &door_seq);
        let mut next = robot1;
        for _ in 0..2 {
            let mut possible_next = vec![];
            for seq in &next {
                possible_next.extend(solve(seq.chars().collect(), &dir_seq));
            }
            let min_len = possible_next.iter().map(|x| x.len()).min().unwrap();
            next = possible_next.into_iter().filter(|x| x.len() == min_len).collect::<Vec<_>>();
        }
        total += next[0].len() * String::from_iter(&line[..line.len()-1]).parse::<usize>().unwrap();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(126384, process("src/test.txt"));
    }
}