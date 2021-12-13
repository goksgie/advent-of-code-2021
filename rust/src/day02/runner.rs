use crate::day02::utilities;
use crate::day02::part01;

/// reads the given input file and produces a Vector of
/// String, i64 pairs. May panic if:
///  a) file is not found,
///  b) format is invalid (STRING PARSABLE_INT)
fn read_input_file(fname: &str) -> Vec<(String, i64)> {
    std::fs::read_to_string(fname)
        .expect(&format!("file not found in the following directory {:?}", std::env::current_dir().unwrap()))
        .lines()
        .map(|line| -> (String, i64) {
                let line_s: Vec<&str> = line.trim().split(" ").collect();
                (String::from(line_s[0]), line_s[1].parse::<i64>().expect("error parsing integer"))
            })
        .collect::<Vec<(String, i64)>>()
}


fn compute_part_1() {
    let raw_instructions = read_input_file("inputs/d02.txt");
    let instructions = utilities::Instruction::construct_instructions(&raw_instructions);
    let result = part01::compute_position(&instructions);

    println!("Day 02 - Part 01: {}", result);
}

fn compute_part_2() {

}

pub fn compute() {
    compute_part_1();
    compute_part_2();
}