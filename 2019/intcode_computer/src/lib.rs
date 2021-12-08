use std::io::{ self, Write };

fn get_parameters(codes: &Vec<i32>, parameters_modes: &[u32; 3], instr_pointer: &usize) -> [i32; 3] {
    let mut parameters: [i32; 3] = [0; 3];
    for j in 0..2 {
        if parameters_modes[2 - j] == 1 {
            parameters[j] = codes[instr_pointer + j + 1] as i32;
        } else {
            let address = codes[instr_pointer + j + 1] as usize;
            parameters[j] = codes[address] as i32;
        }
    }
    parameters[2] = codes[instr_pointer + 3];

    parameters
}

#[derive(Debug, Clone)]
pub struct IntcodeComputer {
    pub program: Vec<i32>,
    pub instr_ptr: usize,
    pub inputs: Vec<i32>,
    pub outputs: Vec<i32>,
    pub return_on_empty_input: bool,
    pub has_halted: bool,
}

impl IntcodeComputer {
    pub fn new(program: Vec<i32>, inputs: Vec<i32>) -> Self {
        Self {
            program,
            instr_ptr: 0,
            inputs,
            outputs: vec![],
            return_on_empty_input: false,
            has_halted: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            let mut instr = self.program[self.instr_ptr].to_string();
            while instr.len() < 5 {
                instr.insert(0, '0');
            }

            let opcode: u32 = instr[3..5].parse().unwrap();
            let mut parameters_modes: [u32; 3] = [0; 3];

            for j in 0..3 {
                parameters_modes[j] = instr.chars().nth(j).unwrap().to_digit(10).unwrap();
            }

            match opcode {
                1 | 2 => {
                    let operands: [i32; 3] = get_parameters(&self.program, &parameters_modes, &self.instr_ptr);

                    let write_pos = self.program[self.instr_ptr + 3] as usize;

                    if opcode == 1 {
                        self.program[write_pos] = operands[0] + operands[1];
                    } else {
                        self.program[write_pos] = operands[0] * operands[1];
                    }

                    self.instr_ptr += 4;
                },
                3 => {
                    let write_pos = self.program[self.instr_ptr + 1] as usize;

                    if self.inputs.is_empty() && self.return_on_empty_input {
                        return;
                    } else if self.inputs.is_empty() {
                        let mut buffer = String::new();

                        print!("Input vector is empty, waiting for input > ");
                        io::stdout().flush().unwrap();

                        io::stdin().read_line(&mut buffer)
                            .expect("Problem reading input!");

                        let input = buffer.trim().parse::<i32>().unwrap();

                        self.program[write_pos] = input;
                    } else {
                        self.program[write_pos] = self.inputs.remove(0);
                    }

                    self.instr_ptr += 2;
                },
                4 => {
                    let output_pos = self.program[self.instr_ptr + 1] as usize;
                    self.outputs.push(self.program[output_pos]);

                    self.instr_ptr += 2;
                },
                5 | 6 => {
                    let parameters: [i32; 3] = get_parameters(&self.program, &parameters_modes, &self.instr_ptr);

                    if opcode == 5 && parameters[0] != 0 {
                        self.instr_ptr = parameters[1] as usize;
                    } else if opcode == 6 && parameters[0] == 0 {
                        self.instr_ptr = parameters[1] as usize;
                    } else {
                        self.instr_ptr += 3;
                    }
                },
                7 | 8 => {
                    let parameters: [i32; 3] = get_parameters(&self.program, &parameters_modes, &self.instr_ptr);

                    if opcode == 7 && parameters[0] < parameters[1] {
                        self.program[parameters[2] as usize] = 1;
                    } else if opcode == 8 && parameters[0] == parameters[1] {
                        self.program[parameters[2] as usize] = 1;
                    } else {
                        self.program[parameters[2] as usize] = 0;
                    }

                    self.instr_ptr += 4;
                },
                99 => {
                    self.has_halted = true;
                    break;
                },
                _ => panic!("Unexpected opcode: {}", opcode),
            }
        }
    }
}
