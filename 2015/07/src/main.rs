use std::collections::HashMap;

fn main() {
    let input = aoc_base::read_input();
    let mut wires: HashMap<&str, String> = HashMap::new();

    for instr in input.lines() {
        let parts = instr.split(" -> ").collect::<Vec<&str>>();
        let input = parts[0];
        let output_wire = parts[1];

        wires.insert(output_wire, input.to_string());
    }

    let wire_a_value = find_wire_value(&mut wires.clone(), "a");
    println!("Part 1: {}", wire_a_value);

    wires.insert("b", wire_a_value.to_string());
    println!("Part 2: {}", find_wire_value(&mut wires, "a"));
}

pub fn find_wire_value(wires: &mut HashMap<&str, String>, wire: &str) -> u16 {
    let wire_res = wires[wire].clone();
    let input_parts = wire_res.split_whitespace().collect::<Vec<&str>>();
   
    let value = if input_parts.len() == 1 {
        input_parts[0].parse::<u16>().unwrap_or_else(|_| find_wire_value(wires, input_parts[0]))
    } else if input_parts.len() == 2 && input_parts[0] == "NOT" {
        !input_parts[1].parse::<u16>().unwrap_or_else(|_| find_wire_value(wires, input_parts[1]))
    } else {
        let operands = [
            input_parts[0].parse::<u16>().unwrap_or_else(|_| find_wire_value(wires, input_parts[0])),
            input_parts[2].parse::<u16>().unwrap_or_else(|_| find_wire_value(wires, input_parts[2])),
        ];

        match input_parts[1] {
            "AND" => operands[0] & operands[1],
            "OR" => operands[0] | operands[1],
            "RSHIFT" => operands[0] >> operands[1],
            "LSHIFT" => operands[0] << operands[1],
            _ => unreachable!()
        }
    };

    *wires.get_mut(wire).unwrap() = value.to_string();
    value
}
