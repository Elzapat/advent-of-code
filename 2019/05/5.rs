use std::fs;
use std::io;
use std::io::Write;

fn main() {

    let mut intcode = fs::read_to_string("5.txt").expect("Reading file error");
    intcode.pop();

    let mut codes = vec![];

    for code in intcode.split(',') {
        codes.push(code.parse::<i32>().unwrap());
    }
    let mut codes_clone = codes.clone();

    println!("Part 1: {}", decode(&mut codes_clone));
    println!("Part 2: {}", decode(&mut codes));
}

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

fn decode(codes: &mut Vec<i32>) -> i32 {

    let mut i = 0;
    let mut last_output = 0;

    loop {
        let mut instr = codes[i].to_string();
        while instr.len() < 5 {
            instr.insert(0, '0');
        }

        let opcode: u32 = instr[3..5].parse().unwrap();
        let mut parameters_modes: [u32; 3] = [0; 3];

        for j in 0..3 {
            parameters_modes[j] = instr.chars().nth(j).unwrap().to_digit(10).unwrap();
        }

        //println!("opcode: {}, instr: {}, i: {}", opcode, instr, i);
        match opcode {
            1 | 2 => {
                let operands: [i32; 3] = get_parameters(&codes, &parameters_modes, &i);

                let write_pos = codes[i + 3] as usize;

                if opcode == 1 {
                    codes[write_pos] = operands[0] + operands[1];
                } else {
                    codes[write_pos] = operands[0] * operands[1];
                }

                i += 4;
            },
            3 => {
                let write_pos = codes[i + 1] as usize;
                let mut buffer = String::new();

                print!("Program input > ");
                io::stdout().flush().unwrap();
                
                io::stdin().read_line(&mut buffer)
                    .expect("Problem reading input!");

                let input = buffer.trim().parse::<i32>().unwrap();

                codes[write_pos] = input;
                
                i += 2;
            },
            4 => {
                let output_pos = codes[i + 1] as usize;
                last_output = codes[output_pos];
                //println!("Program output > {}", codes[output_pos]);

                i += 2;
            },
            5 | 6 => {
                let parameters: [i32; 3] = get_parameters(&codes, &parameters_modes, &i);

                if opcode == 5 && parameters[0] != 0 {
                    i = parameters[1] as usize;
                } else if opcode == 6 && parameters[0] == 0 {
                    i = parameters[1] as usize;
                } else {
                    i += 3;
                }
            },
            7 | 8 => {
                let parameters: [i32; 3] = get_parameters(&codes, &parameters_modes, &i);

                if opcode == 7 && parameters[0] < parameters[1] {
                    codes[parameters[2] as usize] = 1;
                } else if opcode == 8 && parameters[0] == parameters[1] {
                    codes[parameters[2] as usize] = 1;
                } else {
                    codes[parameters[2] as usize] = 0;
                }

                i += 4;
            },
            99 => break,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    last_output
}
