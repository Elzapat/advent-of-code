use std::fs;
use intcode_computer::IntcodeComputer;

fn main() {
    let mut intcode = fs::read_to_string("input.txt").expect("Reading file error");
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
        let mut max_output = 0;

        for i in 0..=4 {
            for j in 0..=4 {
                for k in 0..=4 {
                    for l in 0..=4 {
                        for m in 0..=4 {
                            if !has_unique_elements([i, j, k, l, m]) {
                                continue;
                            }

                            let phase_setting_sequence = [i, j, k, l, m];
                            let mut output = 0;

                            for x in 0..5 {
                                let mut computer = IntcodeComputer::new(codes.clone(), vec![phase_setting_sequence[x], output]);
                                computer.run();
                                output = computer.outputs.pop().unwrap();
                            }

                            if output > max_output {
                                max_output = output;
                            }
                        }
                    }
                }
            }
        }

        println!("Part 1: {}", max_output);
    }

    {
        let mut max_output = 0;

        for i in 5..=9 {
            for j in 5..=9 {
                for k in 5..=9 {
                    for l in 5..=9 {
                        for m in 5..=9 {
                            if !has_unique_elements([i, j, k, l, m]) {
                                continue;
                            }

                            let phase_setting_sequence = [i, j, k, l, m];
                            let mut output = 0;
                            let mut amps = vec![];
                            for x in 0..5 {
                                amps.push(IntcodeComputer {
                                    program: codes.clone(),
                                    instr_ptr: 0,
                                    inputs: vec![phase_setting_sequence[x]],
                                    outputs: vec![],
                                    return_on_empty_input: true,
                                    has_halted: false,
                                });
                            }

                            loop {
                                for x in 0..amps.len() {
                                    amps[x].inputs.push(output);
                                    amps[x].run();

                                    output = amps[x].outputs.pop().unwrap();
                                }

                                if amps[4].has_halted {
                                    break;
                                }
                            }

                            if output > max_output {
                                max_output = output;
                            }
                        }
                    }
                }
            }
        }

        println!("Part 2: {}", max_output);
    }
}
