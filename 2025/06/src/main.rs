fn main() {
    let input = aoc_base::read_input();
    let mut math = vec![];
    let mut operands = vec![];
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for (i, line) in input.lines().enumerate() {
        if i == input.lines().count() - 1 {
            operands = line.split_whitespace().collect::<Vec<&str>>();
            continue;
        }

        math.push(
            line.split_whitespace()
                .map(|m| m.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    for i in 0..math[0].len() {
        let operand = operands[i];
        let mut res = if operand == "*" { 1 } else { 0 };

        for j in 0..math.len() {
            if operand == "*" {
                res *= math[j][i];
            } else {
                res += math[j][i];
            }
        }

        result_p1 += res;
    }

    let lines = input.lines().collect::<Vec<&str>>();
    let mut max_line_len = 0;

    for line in &lines {
        if line.len() > max_line_len {
            max_line_len = line.len();
        }
    }

    let mut operand_i = 0;
    let mut res = if operands[operand_i] == "*" { 1 } else { 0 };

    for i in 0..max_line_len {
        let mut skip = true;
        for j in 0..lines.len() - 1 {
            if lines[j].chars().nth(i).unwrap() != ' ' {
                skip = false;
                break;
            }
        }

        if skip {
            result_p2 += res;
            operand_i += 1;
            res = if operands[operand_i] == "*" { 1 } else { 0 };
            continue;
        }

        let mut number = String::new();
        for j in 0..lines.len() - 1 {
            if lines[j].chars().nth(i).unwrap() != ' ' {
                number.push(lines[j].chars().nth(i).unwrap());
            }
        }

        if operands[operand_i] == "*" {
            res *= number.parse::<u64>().unwrap();
        } else {
            res += number.parse::<u64>().unwrap();
        }
    }

    result_p2 += res;

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}
