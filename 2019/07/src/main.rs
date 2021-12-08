use std::fs;
use intcode_computer::IntcodeComputer;

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

                            output = IntcodeComputer::new(codes.clone(), vec![i, output]).outputs.pop().unwrap();
                            output = IntcodeComputer::new(codes.clone(), vec![j, output]).outputs.pop().unwrap();
                            output = IntcodeComputer::new(codes.clone(), vec![k, output]).outputs.pop().unwrap();
                            output = IntcodeComputer::new(codes.clone(), vec![l, output]).outputs.pop().unwrap();
                            output = IntcodeComputer::new(codes.clone(), vec![m, output]).outputs.pop().unwrap();

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

                            let amps = [IntcodeComputer {
                                program: codes.clone(),
                                instr_ptr: 0,
                                inputs: vec![],
                                outputs: vec![],
                                return_on_empty_input: true,
                                has_halted: false,
                            }; 5];

                            loop {
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
