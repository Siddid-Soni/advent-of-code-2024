use day_17::part1::process;
use code_timing_macros::time_snippet;

fn main() {
    let result = time_snippet!(process());
    println!("{result:?}");
}