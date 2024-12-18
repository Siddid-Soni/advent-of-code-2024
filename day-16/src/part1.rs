use crate::parse_input;
use std::path::Path;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const DIRS: [(i32, i32);4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
    direc: usize,
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

pub fn process<P>(filename: P) -> Option<usize> where P: AsRef<Path>  {
    let grid = parse_input(filename);

    let (mut sr, mut sc) = (0, 0);

    'outer: for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                sr = row;
                sc = col;
                break 'outer;
            }
        }
    }
    let w = grid[0].len();
    let mut dist = vec![usize::MAX; grid.len() * grid[0].len() * 4];

    let mut heap = BinaryHeap::new();

    dist[(sr * w + sc) * 4] = 0;
    heap.push(State {cost: 0, pos: (sr, sc), direc: 0});

    while let Some(State { cost, pos, direc }) = heap.pop() {
        if grid[pos.0][pos.1] == 'E' { return Some(cost); }
        if cost > dist[(pos.0 * w + pos.1) * 4 + direc] { continue; }

        for next_state in [State {cost: cost + 1000, pos, direc: (direc + 1) % 4}, State {cost : cost + 1000, pos, direc: (direc + 3) % 4}, 
        State {cost: cost + 1, pos: ((pos.0 as i32 + DIRS[direc].0) as usize, (pos.1 as i32 + DIRS[direc].1) as usize), direc}] {
            if grid[next_state.pos.0][next_state.pos.1] == '#' { continue; }

            if next_state.cost < dist[(next_state.pos.0 * w + next_state.pos.1) * 4 + next_state.direc] {
                dist[(next_state.pos.0 * w + next_state.pos.1) * 4 + next_state.direc] = next_state.cost;
                heap.push(next_state);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(Some(7036), process("src/test.txt"));
    }
}