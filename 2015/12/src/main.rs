use serde_json::Value;

fn main() {
    let input = aoc_base::read_input();

    let mut sum = 0;

    let mut chars = input.chars();
    let mut char = chars.next();

    while char != None {
        // println!("{char:?}");
        let mut number = "".to_string();

        if char.unwrap().is_numeric() || char.unwrap() == '-' {
            while char.unwrap().is_numeric() || char.unwrap() == '-' {
                if let Some(d) = char {
                    number.push(d);
                }

                char = chars.next();

                if char.is_none() {
                    break;
                }
            }

            sum += number.parse::<i32>().unwrap();
            number.clear();
        }

        char = chars.next();
    }

    println!("Part 1: {sum}");

    let json: Value = serde_json::from_str(&input).unwrap();

    println!("Part 2: {}", count_numbers(&json));
}

fn count_numbers(object: &Value) -> i64 {
    match object {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(|v| count_numbers(v)).sum(),
        Value::Object(obj) if obj.values().any(|v| *v == Value::String("red".to_string())) => 0,
        Value::Object(obj) => {
            count_numbers(&Value::Array(obj.values().map(|v| v.clone()).collect()))
        }
        _ => 0,
    }
}
