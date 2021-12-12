use std::io::{ self, Write };

#[derive(Debug, Clone)]
pub struct IntcodeComputer {
    pub program: Vec<i64>,
    pub instr_ptr: usize,
    pub relative_base: isize,
    pub inputs: Vec<i64>,
    pub outputs: Vec<i64>,
    pub return_on_empty_input: bool,
    pub has_halted: bool,
}

impl IntcodeComputer {
    pub fn new(mut program: Vec<i64>, inputs: Vec<i64>) -> Self {
        program.append(&mut vec![0; 1024]);
        Self {
            program,
            instr_ptr: 0,
            relative_base: 0,
            inputs,
            outputs: vec![],
            return_on_empty_input: false,
            has_halted: false,
        }
    }

    fn get_parameters(&self, opcode: u32, parameters_modes: &[u32; 3]) -> [i64; 3] {
        let mut parameters: [i64; 3] = [0; 3];
        let number_params = match opcode {
            9 => 1,
            5 | 6  => 2,
            1 | 2 | 3 | 4 | 7 | 8 => 3,
            _ => panic!("Unexpected opcode: {}", opcode),
        };

        for j in 0..number_params {
            // println!("j = {}, {:?}", j, parameters_modes);
            match parameters_modes[2 - j] {
                // Position mode
                0 => {
                    let address = self.program[self.instr_ptr + j + 1];

                    if j == 2 {
                        parameters[j] = address as i64;
                    } else if (opcode == 3 || opcode == 4) && j == 0 {
                        parameters[j] = address as i64;
                    } else {
                        parameters[j] = self.program[address as usize];
                    }
                },
                // Immediate mode
                1 => parameters[j] = self.program[self.instr_ptr + j + 1],
                // Relative mode
                2 => {
                    let parameter = self.program[self.instr_ptr + j + 1];
                    let address = (self.relative_base + parameter as isize) as usize;

                    if j == 2 {
                        parameters[j] = address as i64;
                    } else if (opcode == 3 || opcode == 4) && j == 0 {
                        parameters[j] = address as i64;
                    } else {
                        parameters[j] = self.program[address as usize];
                    }
                },
                _ => panic!("Unexpected parameter mode: {}", parameters_modes[2 - j]),
            }
        }

        parameters
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
            // println!("opcode = {}, parameter_modes = {:?}", opcode, parameters_modes);

            match opcode {
                1 | 2 => {
                    let params: [i64; 3] = self.get_parameters(opcode, &parameters_modes);
                    println!("{:?}", params);

                    // let write_pos = self.program[self.instr_ptr + 3] as usize;
                    let write_pos = params[2] as usize;

                    if opcode == 1 {
                        println!("modes = {:?}, params = {:?}", parameters_modes, params);
                        println!("ptr = {}, {} + {} = {} at address {}", self.instr_ptr, params[0], params[1], params[0] + params[1], params[2]);
                        self.program[write_pos] = params[0] + params[1];
                    } else {
                        self.program[write_pos] = params[0] * params[1];
                    }

                    self.instr_ptr += 4;
                },
                3 => {
                    // let write_pos = self.program[self.instr_ptr + 1] as usize;
                    let params = self.get_parameters(opcode, &parameters_modes);
                    // let write_pos = params[0] as usize;
                    let write_pos = params[0] as usize;
                    let parameter = self.program[self.instr_ptr + 1];
                    let address = (self.relative_base + parameter as isize) as usize;
                    let write_pos = address;

                    if self.inputs.is_empty() && self.return_on_empty_input {
                        return;
                    } else if self.inputs.is_empty() {
                        let mut buffer = String::new();

                        print!("Input vector is empty, waiting for input > ");
                        io::stdout().flush().unwrap();

                        io::stdin().read_line(&mut buffer)
                            .expect("Problem reading input!");

                        let input = buffer.trim().parse::<i64>().unwrap();

                        self.program[write_pos] = input;
                    } else {
                        self.program[write_pos] = self.inputs.remove(0);
                    }

                    self.instr_ptr += 2;
                },
                4 => {
                    let output_pos = self.get_parameters(opcode, &parameters_modes)[0];
                    self.outputs.push(output_pos);

                    self.instr_ptr += 2;
                },
                5 | 6 => {
                    let parameters: [i64; 3] = self.get_parameters(opcode, &parameters_modes);

                    if opcode == 5 && parameters[0] != 0 {
                        self.instr_ptr = parameters[1] as usize;
                    } else if opcode == 6 && parameters[0] == 0 {
                        self.instr_ptr = parameters[1] as usize;
                    } else {
                        self.instr_ptr += 3;
                    }
                },
                7 | 8 => {
                    let parameters: [i64; 3] = self.get_parameters(opcode, &parameters_modes);

                    if opcode == 7 && parameters[0] < parameters[1] {
                        self.program[parameters[2] as usize] = 1;
                    } else if opcode == 8 && parameters[0] == parameters[1] {
                        self.program[parameters[2] as usize] = 1;
                    } else {
                        self.program[parameters[2] as usize] = 0;
                    }

                    self.instr_ptr += 4;
                },
                9 => {
                    let parameters = self.get_parameters(opcode, &parameters_modes);
                    self.relative_base += parameters[0] as isize;

                    self.instr_ptr += 2;
                }
                99 => {
                    self.has_halted = true;
                    break;
                },
                _ => panic!("Unexpected opcode: {}", opcode),
            }
        }
    }
}
