use day_18::part1::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process("day-18/input1.txt", 1024, (70,70)));
    println!("{}", result);
}