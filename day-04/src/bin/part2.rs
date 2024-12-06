use day_04::part2::process;
use code_timing_macros::time_snippet;

fn main() {

    let result = time_snippet!(process("day-04/input1.txt"));
    println!("{}", result);
}
