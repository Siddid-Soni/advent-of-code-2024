use day_10::part2::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process("day-10/input1.txt"));
    println!("{}", result);
}