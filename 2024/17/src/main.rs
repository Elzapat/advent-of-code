fn main() {
    let ra = 65804993_u64;

    let program = "2,4,1,1,7,5,1,4,0,3,4,5,5,5,3,0"
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!(
        "Part 1: {}",
        run_program(&program, ra)
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    // This initial value was found by trial and error starting from when the program
    // outputs 16 values, the same length as the program.
    // Since the digit have a cycle and increment like a number, I started by searching a
    // number where the last digit was right, then increased a bit to get the 15th number right
    // and so on until I found the right input to get the program in the output
    let mut ra = 202322936856672;

    loop {
        if program == run_program(&program, ra) {
            println!("Part 2: {ra}");
            break;
        }

        ra += 1;
    }
}

fn run_program(program: &[u64], mut ra: u64) -> Vec<u64> {
    let mut rb = 0;
    let mut rc = 0;
    let mut instrptr = 0;
    let mut outputs = vec![];

    while instrptr < program.len() {
        let (literal_operand, combo_operand) = match program[instrptr + 1] {
            value if (0..=3).contains(&value) => (value, value),
            4 => (4, ra),
            5 => (5, rb),
            6 => (6, rc),
            _ => panic!("Invalid operand {}", program[instrptr + 1]),
        };

        match program[instrptr] {
            // adv
            0 => {
                // supposed to be truncated
                ra /= 2_u64.pow(combo_operand as u32);
                instrptr += 2;
            }
            // bxl
            1 => {
                rb ^= literal_operand;
                instrptr += 2;
            }
            // bst
            2 => {
                rb = combo_operand % 8;
                instrptr += 2;
            }
            // jnz
            3 => {
                if ra != 0 {
                    instrptr = literal_operand as usize;
                } else {
                    instrptr += 2;
                }
            }
            // bxc
            4 => {
                rb ^= rc;
                instrptr += 2;
            }
            // out
            5 => {
                outputs.push(combo_operand % 8);
                instrptr += 2;
            }
            // bdv
            6 => {
                rb = ra / 2_u64.pow(combo_operand as u32);
                instrptr += 2;
            }
            // cdv
            7 => {
                rc = ra / 2_u64.pow(combo_operand as u32);
                instrptr += 2;
            }
            opcode => panic!("Invalid opcode {opcode}"),
        }
    }

    outputs
}
