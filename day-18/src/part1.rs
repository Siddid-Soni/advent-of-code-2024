use crate::parse_input;
use std::path::Path;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_min_cost(grid: &Vec<Vec<char>>, start: (i32, i32), goal: (i32,i32)) -> Option<usize> {
    let w = grid[0].len();
    let mut dist = vec![usize::MAX; grid.len() * grid[0].len()];
    let mut heap = BinaryHeap::new();
    heap.push(State {cost: 0, pos: start});
    dist[(start.0 * w as i32 + start.1) as usize] = 0;
    
    while let Some(State { cost, pos }) = heap.pop() {
        if pos == goal {return Some(cost);}
        if cost > dist[(pos.0 * w as i32 + pos.1) as usize] {continue;}

        for next in [State {cost: cost+1, pos: (pos.0+1, pos.1)}, State {cost: cost+1, pos: (pos.0, pos.1+1)},
        State {cost: cost+1, pos: (pos.0 - 1, pos.1)}, State {cost: cost+1, pos: (pos.0, pos.1 - 1)}] {
            if next.pos.0 < 0 || next.pos.0>=grid.len() as i32 || next.pos.1 < 0 || next.pos.1>=grid[0].len() as i32 
            || grid[next.pos.0 as usize][next.pos.1 as usize] == '#' {continue;}

            if next.cost < dist[(next.pos.0 * w as i32 + next.pos.1) as usize] {
                dist[(next.pos.0 * w as i32 + next.pos.1) as usize] = next.cost;
                heap.push(next);
            }
        }
    }
    None
}

pub fn process(filename: impl AsRef<Path>, b:usize, end: (i32,i32)) -> usize {
    let corrupt = parse_input(filename);
    let mut grid = vec![vec!['.';end.0 as usize+1];end.1 as usize +1];

    for i in corrupt[..b].iter() {
        grid[i.0][i.1] = '#';
    }
    
    find_min_cost(&grid, (0,0), end).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(22, process("src/test.txt", 12, (6,6)));
    }
}