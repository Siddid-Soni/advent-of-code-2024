use day_14::part2::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process("day-14/input1.txt"));
    println!("{}", result);
}
