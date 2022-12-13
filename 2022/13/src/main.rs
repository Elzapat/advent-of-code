use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Integer(i32),
    List(Vec<Value>),
}

#[derive(Debug, PartialEq)]
enum ValueCompare {
    RightOrder,
    WrongOrder,
    Equal,
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut lines = input.lines().map(|l| l.to_owned());

    let mut correct_pairs = 0;
    let mut pair_index = 1;

    while let Some(p1_raw) = lines.next() {
        let p2_raw = lines.next().unwrap();

        let p1 = parse_value(&p1_raw);
        let p2 = parse_value(&p2_raw);

        if let ValueCompare::RightOrder = compare_values(&p1, &p2) {
            correct_pairs += pair_index;
        }

        lines.next();
        pair_index += 1;
    }

    println!("Part 1: {correct_pairs}");

    let mut packets = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        packets.push(parse_value(line));
    }

    let divider1 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
    let divider2 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);

    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort_by(|p1, p2| match compare_values(p1, p2) {
        ValueCompare::WrongOrder => Ordering::Greater,
        ValueCompare::RightOrder => Ordering::Less,
        ValueCompare::Equal => Ordering::Equal,
    });

    println!(
        "Part 2: {}",
        (packets.iter().position(|p| p == &divider1).unwrap() + 1)
            * (packets.iter().position(|p| p == &divider2).unwrap() + 1)
    );
}

fn compare_values(v1: &Value, v2: &Value) -> ValueCompare {
    match v1 {
        Value::Integer(int1) => match v2 {
            Value::Integer(int2) => match int1.cmp(int2) {
                Ordering::Less => ValueCompare::RightOrder,
                Ordering::Greater => ValueCompare::WrongOrder,
                Ordering::Equal => ValueCompare::Equal,
            },
            Value::List(_) => compare_values(&Value::List(vec![Value::Integer(*int1)]), v2),
        },
        Value::List(list1) => match v2 {
            Value::Integer(_) => compare_values(v1, &Value::List(vec![v2.clone()])),
            Value::List(list2) => match list1.get(0) {
                Some(v1) => match list2.get(0) {
                    Some(v2) => match compare_values(v1, v2) {
                        ValueCompare::Equal => compare_values(
                            &Value::List(list1[1..].to_vec()),
                            &Value::List(list2[1..].to_vec()),
                        ),
                        compare => compare,
                    },
                    None => ValueCompare::WrongOrder,
                },
                None => match list2.get(0) {
                    Some(_) => ValueCompare::RightOrder,
                    None => ValueCompare::Equal,
                },
            },
        },
    }
}

fn parse_value(input: &str) -> Value {
    if input.is_empty() {
        return Value::List(vec![]);
    }

    if let Ok(int) = input.parse() {
        return Value::Integer(int);
    }

    let input = input[1..input.len() - 1].trim();

    if input.is_empty() {
        return Value::List(vec![]);
    }

    if let Ok(int) = input.parse() {
        return Value::List(vec![Value::Integer(int)]);
    }

    let values = input.split(',').collect::<Vec<&str>>();
    let mut list = vec![];

    let mut i = 0;
    while i < values.len() {
        let value = values[i];

        if value.starts_with('[') {
            if value == "[]" {
                list.push(Value::List(vec![]));
            } else {
                let mut nested_level = 0;
                let mut temp_val = "".to_owned();

                loop {
                    for c in values[i].chars() {
                        if c == ']' {
                            nested_level -= 1;
                        } else if c == '[' {
                            nested_level += 1;
                        }
                    }

                    if nested_level == 0 {
                        break;
                    }

                    temp_val.push_str(values[i]);
                    temp_val.push(',');
                    i += 1;
                }
                temp_val.push_str(values[i]);

                list.push(parse_value(&temp_val));
            }
        } else {
            list.push(parse_value(value));
        }

        i += 1;
    }

    Value::List(list)
}
