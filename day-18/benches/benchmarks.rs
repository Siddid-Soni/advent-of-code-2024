use day_18::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process("input1.txt", 1024, (70,70));
}

#[divan::bench]
fn part2() {
    part2::process("input2.txt", (70,70));
}