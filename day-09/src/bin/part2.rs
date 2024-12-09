use day_09::part2::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process("day-09/input1.txt"));
    println!("{}", result);
}
