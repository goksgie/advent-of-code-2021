/// An enumation to hold defined instructions
pub enum Instruction {
    /// The number of **positive** steps in the horizontal space 
    Forward(i64),

    // The number of **positive** steps in the vertical space
    Up(i64),

    // The number of **negative** steps in the vertical space (the value is still psotive.)
    Down(i64),
}

impl Instruction {
    /// Given horizontal and vertical positions, applies the instruction
    /// If the vertical value is smaller than zero after the operation,
    /// sets it to zero.
    pub fn apply(&self, horizontal_p: &mut i64, vertical_p: &mut i64) {
        match self {
            Self::Down(val)    => {*vertical_p += val;},
            Self::Up(val)      => {*vertical_p -= val;}, 
            Self::Forward(val) => {*horizontal_p += val;},
        }
        
        if *vertical_p < 0 {
            *vertical_p = 0;
        }
    }

    /// Given the Vector of raw instructions, converts them into
    /// more code friendly enum type. May panic if the instruction
    /// is invalid. 
    pub fn construct_instructions(raw_instructions: &Vec<(String, i64)>) -> Vec<Self> {
        let mut instructions: Vec<Instruction> = Vec::new();

        for (instruction, value) in raw_instructions {

            if *value < 0 {
                panic!("The instruction value cannot be negative: {}", *value);
            }

            match instruction.to_lowercase().as_str() {
                "forward" => instructions.push(Instruction::Forward(*value)),
                "up"      => instructions.push(Instruction::Up(*value)),
                "down"    => instructions.push(Instruction::Down(*value)),
                _         => panic!("Unsupported instruction: {}", *instruction),
            }

        }

        instructions
    }
}