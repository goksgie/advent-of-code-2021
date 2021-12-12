use crate::day01::*;

/// read the given input file and produce vector of unsigned
/// integers. The expected file's format should be one integer
/// per line.
fn read_input_file(fname: &str) -> Vec<u64> {
    std::fs::read_to_string(fname)
        .expect(&format!("file not found in the following directory {:?}", std::env::current_dir().unwrap()))
        .lines()
        .map(|line| line.trim().parse::<u64>().expect("error parsing integer"))
        .collect::<Vec<u64>>()
}


fn compute_part_1() {
    let result = part01::compute_consequtive_increase(&read_input_file("inputs/d01.txt"));
    println!("Day 01 - Part 01: {}", result);
}

fn compute_part_2() {
    let result = part02::compute_increase_with_window(&read_input_file("inputs/d01.txt"), 3);
    println!("Day 01 - Part 02: {}", result);
}

pub fn compute() {
    compute_part_1();
    compute_part_2();
}