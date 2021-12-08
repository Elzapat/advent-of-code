use std::fs;
use std::io;
use std::io::Write;

fn main() {
    let mut intcode = fs::read_to_string("input.ex3.txt").expect("Reading file error");
    intcode.pop();

    let mut codes = vec![];

    for code in intcode.split(',') {
        codes.push(code.parse::<i32>().unwrap());
    }

    // https://stackoverflow.com/questions/46766560/how-to-check-if-there-are-duplicates-in-a-slice
    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + std::hash::Hash,
    {
        let mut uniq = std::collections::HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    {
        let mut output = 0;
        let mut max_output = 0;

        for i in 0..=4 {
            for j in 0..=4 {
                for k in 0..=4 {
                    for l in 0..=4 {
                        for m in 0..=4 {
                            if !has_unique_elements([i, j, k, l, m]) {
                                continue;
                            }

                            output = decode(&mut codes.clone(), vec![i, output]).0;
                            output = decode(&mut codes.clone(), vec![j, output]).0;
                            output = decode(&mut codes.clone(), vec![k, output]).0;
                            output = decode(&mut codes.clone(), vec![l, output]).0;
                            output = decode(&mut codes.clone(), vec![m, output]).0;
                            if output > max_output {
                                max_output = output;
                            }
                            output = 0;
                        }
                    }
                }
            }
        }

        println!("Part 1: {}", max_output);
    }

    {
        let mut output = 0;
        let mut max_output = 0;

        for i in 5..=9 {
            for j in 5..=9 {
                for k in 5..=9 {
                    for l in 5..=9 {
                        for m in 5..=9 {
                            if !has_unique_elements([i, j, k, l, m]) {
                                continue;
                            }

                            let mut a_mem = codes.clone();
                            let mut b_mem = codes.clone();
                            let mut c_mem = codes.clone();
                            let mut d_mem = codes.clone();
                            let mut e_mem = codes.clone();

                            loop {
                                // output = decode(&mut a_mem, vec![i, output]).0;
                                // output = decode(&mut b_mem, vec![j, output]).0;
                                // output = decode(&mut c_mem, vec![k, output]).0;
                                // output = decode(&mut d_mem, vec![l, output]).0;
                                let (out, halted) = decode(&mut a_mem, vec![i, output]);
                                println!("a{},{}", out, halted);
                                let (out, halted) = decode(&mut b_mem, vec![j, out]);
                                println!("b{},{}", out, halted);
                                let (out, halted) = decode(&mut c_mem, vec![k, out]);
                                println!("c{},{}", out, halted);
                                let (out, halted) = decode(&mut d_mem, vec![l, out]);
                                println!("d{},{}", out, halted);
                                let (out, halted) = decode(&mut e_mem, vec![m, out]);
                                output = out;
                                if halted {
                                    break;
                                }
                                println!("{},{}", out, halted);
                            }

                            if output > max_output {
                                max_output = output;
                            }

                            output = 0;
                        }
                    }
                }
            }
        }

        println!("Part 2: {}", max_output);
    }
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

fn decode(codes: &mut Vec<i32>, mut inputs: Vec<i32>) -> (i32, bool) {
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

                if inputs.is_empty() {
                    return (last_output, false);
                    let mut buffer = String::new();

                    print!("Input vector is empty, waiting for input > ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut buffer)
                        .expect("Problem reading input!");

                    let input = buffer.trim().parse::<i32>().unwrap();

                    codes[write_pos] = input;
                } else {
                    codes[write_pos] = inputs.remove(0);
                }

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

    (last_output, true)
}
