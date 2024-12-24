use std::collections::HashMap;

fn main() {
    let input = aoc_base::read_input();
    let mut gates_input = false;
    let mut wires = HashMap::new();
    let mut gates = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            gates_input = true;
            continue;
        }

        if gates_input {
            let split = line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            gates.insert(
                split[4].clone(),
                (split[0].clone(), split[1].clone(), split[2].clone()),
            );
        } else {
            let mut split = line.split(":");
            let name = split.next().unwrap().to_string();
            let value = split.next().unwrap().trim().parse::<u64>().unwrap();
            wires.insert(name, value);
        }
    }

    println!("Part 1: {}", get_value_of_wire_type('z', &wires, &gates));

    let mut wrong_gates = vec![];

    for (result, (op1, operator, op2)) in &gates {
        if operator == "XOR" {
            if ((op1.starts_with('x') && op2.starts_with('y'))
                || (op1.starts_with('y') && op2.starts_with('x')))
                && result.starts_with('z')
                && result != "z00"
            {
                wrong_gates.push(result.clone());
            }

            if !op1.starts_with('x')
                && !op1.starts_with('y')
                && !op2.starts_with('x')
                && !op2.starts_with('y')
                && !result.starts_with('z')
            {
                wrong_gates.push(result.clone());
            }

            for (op1_2, operator_2, op2_2) in gates.values() {
                if (result == op1_2 || result == op2_2) && operator_2 == "OR" {
                    wrong_gates.push(result.clone());
                }
            }
        } else if result.starts_with('z') && operator != "XOR" && result != "z45" {
            wrong_gates.push(result.clone());
        } else if operator == "AND" && op1 != "x00" && op2 != "x00" {
            for (op1_2, operator_2, op2_2) in gates.values() {
                if (result == op1_2 || result == op2_2) && operator_2 != "OR" {
                    wrong_gates.push(result.clone());
                }
            }
        }
    }

    wrong_gates.dedup();
    wrong_gates.sort();
    println!("Part 2: {}", wrong_gates.join(","));
}

fn get_value_of_wire_type(
    wire_type: char,
    wires: &HashMap<String, u64>,
    gates: &HashMap<String, (String, String, String)>,
) -> u64 {
    let mut result = 0_u64;
    let mut all_wires = gates.keys().collect::<Vec<_>>();
    all_wires.extend(wires.keys().collect::<Vec<_>>());

    for output_wire in all_wires {
        if !output_wire.starts_with(wire_type) {
            continue;
        }

        let n = output_wire[1..].parse::<u64>().unwrap();

        result += get_wire_value(output_wire.clone(), wires, gates) << n;
    }

    result
}

fn get_wire_value(
    wire: String,
    wires: &HashMap<String, u64>,
    gates: &HashMap<String, (String, String, String)>,
) -> u64 {
    if let Some(value) = wires.get(&wire) {
        return *value;
    }

    let (op1, operator, op2) = &gates[&wire];

    let op1 = get_wire_value(op1.clone(), wires, gates);
    let op2 = get_wire_value(op2.clone(), wires, gates);

    match &operator[..] {
        "AND" => op1 & op2,
        "OR" => op1 | op2,
        "XOR" => op1 ^ op2,
        _ => unreachable!(),
    }
}
