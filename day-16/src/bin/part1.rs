use day_16::part1::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process("day-16/input1.txt").unwrap());
    println!("{}", result);
}