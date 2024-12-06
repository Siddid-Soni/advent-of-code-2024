use day_03::part1::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process("day-03/input1.txt"));
    println!("{}", result);
}