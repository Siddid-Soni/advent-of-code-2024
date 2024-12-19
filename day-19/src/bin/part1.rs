use day_19::part1::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process("day-19/input1.txt"));
    println!("{}", result);
}