use crate::day02::utilities::Instruction;

pub fn compute_position(instructions: &Vec<Instruction>) -> i64 {
    let mut horizontal_p = 0;
    let mut vertical_p   = 0;

    for instruction in instructions {
        instruction.apply(&mut horizontal_p, &mut vertical_p);
    }

    horizontal_p * vertical_p
}


#[test]
fn test_compute_position() {
    let test_data: Vec<(Vec<(String, i64)>, i64)> = vec![
        (
            vec![
                (String::from("Forward"), 5),
                (String::from("down"), 5),
                (String::from("Forward"), 8),
                (String::from("up"), 3),
                (String::from("down"), 8),
                (String::from("Forward"), 2)
            ], 
            150
        ),
    ];

    for (test_inp, result) in &test_data {
        let instructions = Instruction::construct_instructions(test_inp);
        assert_eq!(compute_position(&instructions), *result);
    }
}