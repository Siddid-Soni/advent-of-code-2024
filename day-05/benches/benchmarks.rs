use day_05::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process("input1.txt");
}

#[divan::bench]
fn part2() {
    part2::process("input2.txt");
}