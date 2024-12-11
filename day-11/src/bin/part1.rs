use day_11::part1::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process("day-11/input1.txt"));
    println!("{}", result);
}