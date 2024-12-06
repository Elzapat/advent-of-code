use std::collections::HashMap;
fn main() {
    let instructions = aoc_base::read_input()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let mut iptr = 0;
    let mut registers = HashMap::new();
    registers.insert("a".to_string(), 1);

    while iptr >= 0 && iptr < instructions.len() as i32 {
        let instruction = &instructions[iptr as usize];
        let (instr, arg1, arg2) = parse_instruction(&instruction);

        match &instr[..] {
            "hlf" => {
                *registers.entry(arg1).or_insert(0) /= 2;
                iptr += 1
            }
            "tpl" => {
                *registers.entry(arg1).or_insert(0) *= 3;
                iptr += 1
            }
            "inc" => {
                *registers.entry(arg1).or_insert(0) += 1;
                iptr += 1
            }
            "jmp" => {
                let offset = arg1.parse::<i32>().unwrap();
                iptr += offset;
            }
            "jie" => {
                let register_value = registers.entry(arg1).or_insert(0);
                let offset = arg2.unwrap().parse::<i32>().unwrap();
                iptr += if *register_value % 2 == 0 { offset } else { 1 };
            }
            "jio" => {
                let register_value = registers.entry(arg1).or_insert(0);
                let offset = arg2.unwrap().parse::<i32>().unwrap();
                iptr += if *register_value == 1 { offset } else { 1 };
            }
            _ => unreachable!(),
        }
    }

    println!("Part 2: {}", registers[&"b".to_string()]);
}

fn parse_instruction(instruction: &String) -> (String, String, Option<String>) {
    let instr = instruction[..3].to_string();
    let mut args = instruction[4..].split(", ");

    (
        instr,
        args.next().unwrap().to_string(),
        args.next().map(|a| a.to_string()),
    )
}
