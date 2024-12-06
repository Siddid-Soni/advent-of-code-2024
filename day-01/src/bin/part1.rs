use day_01::part1::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process("day-01/input1.txt"));
    println!("{}", result);
}