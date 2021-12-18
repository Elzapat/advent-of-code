use serde_json::Value;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let input = file
        .lines()
        .map(|line| serde_json::from_str::<Value>(&line).unwrap())
        .collect::<Vec<Value>>();

    {
        let mut current_sum = input[0].clone();

        for i in 1..input.len() {
            current_sum = snailfish_number_addition(current_sum, input[i].clone());
        }

        println!("Part 1: {}", snailfish_number_magnitude(current_sum));
    }

    {
        let mut max_magnitude = 0;

        for i in 0..input.len() {
            for j in 0..input.len() {
                if i == j {
                    continue;
                }

                let sum = snailfish_number_addition(input[i].clone(), input[j].clone());
                let magnitude = snailfish_number_magnitude(sum);

                if magnitude > max_magnitude {
                    max_magnitude = magnitude;
                }
            }
        }

        println!("Part 2: {}", max_magnitude);
    }
}

fn snailfish_number_magnitude(number: Value) -> u64 {
    if number.is_array() {
        let arr = number.as_array().unwrap();
        3 * snailfish_number_magnitude(arr[0].clone()) + 2 * snailfish_number_magnitude(arr[1].clone())
    } else {
        number.as_u64().unwrap()
    }
}

#[test]
fn magnitude() {
    assert_eq!(snailfish_number_magnitude(serde_json::from_str("[9,1]").unwrap()), 29);
    assert_eq!(snailfish_number_magnitude(serde_json::from_str("[[9,1],[1,9]]").unwrap()), 129);
    assert_eq!(snailfish_number_magnitude(serde_json::from_str("[[1,2],[[3,4],5]]").unwrap()), 143);
    assert_eq!(snailfish_number_magnitude(serde_json::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap()), 1384);
    assert_eq!(snailfish_number_magnitude(serde_json::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap()), 3488);
}

fn snailfish_number_addition(number1: Value, number2: Value) -> Value {
    let mut number = serde_json::to_string(&vec![number1, number2]).unwrap();

    let mut nesting;
    'main: loop {
        nesting = 0;

        // Check for explosion
        for i in 0..number.len() {
            if &number[i..i+1] == "[" {
                nesting += 1;
            } else if &number[i..i+1] == "]" {
                nesting -= 1;
            }

            if nesting >= 5 && number[i+1..i+2].parse::<u32>().is_ok() {
                let first_number;
                let second_number;

                if number[i+1..i+4].parse::<u32>().is_ok() {
                    first_number = number[i+1..i+4].parse::<u32>().unwrap();

                    if number[i+4..i+8].parse::<u32>().is_ok() {
                        second_number = number[i+5..i+8].parse::<u32>().unwrap();
                        number.replace_range(i..i+9, "0");
                    } else if number[i+4..i+7].parse::<u32>().is_ok() {
                        second_number = number[i+5..i+7].parse::<u32>().unwrap();
                        number.replace_range(i..i+8, "0");
                    } else {
                        second_number = number[i+5..i+6].parse::<u32>().unwrap();
                        number.replace_range(i..i+7, "0");
                    }
                } else if number[i+1..i+3].parse::<u32>().is_ok() {
                    first_number = number[i+1..i+3].parse::<u32>().unwrap();

                    if number[i+4..i+7].parse::<u32>().is_ok() {
                        second_number = number[i+4..i+7].parse::<u32>().unwrap();
                        number.replace_range(i..i+8, "0");
                    } else if number[i+4..i+6].parse::<u32>().is_ok() {
                        second_number = number[i+4..i+6].parse::<u32>().unwrap();
                        number.replace_range(i..i+7, "0");
                    } else {
                        second_number = number[i+4..i+5].parse::<u32>().unwrap();
                        number.replace_range(i..i+6, "0");
                    }
                } else {
                    first_number = number[i+1..i+2].parse::<u32>().unwrap();

                    if number[i+3..i+6].parse::<u32>().is_ok() {
                        second_number = number[i+3..i+6].parse::<u32>().unwrap();
                        number.replace_range(i..i+7, "0");
                    } else if number[i+3..i+5].parse::<u32>().is_ok() {
                        second_number = number[i+3..i+5].parse::<u32>().unwrap();
                        number.replace_range(i..i+6, "0");
                    } else {
                        second_number = number[i+3..i+4].parse::<u32>().unwrap();
                        number.replace_range(i..i+5, "0");
                    }
                }

                for j in (0..i).rev() {
                    if j >= 2 && number[j-2..=j].parse::<u32>().is_ok() {
                        number.replace_range(j-2..=j, &(number[j-2..=j].parse::<u32>().unwrap() + first_number).to_string());
                        break;
                    } else if j >= 1 && number[j-1..=j].parse::<u32>().is_ok() {
                        number.replace_range(j-1..=j, &(number[j-1..=j].parse::<u32>().unwrap() + first_number).to_string());
                        break;
                    } else if number[j..j+1].parse::<u32>().is_ok() {
                        number.replace_range(j..j+1, &(number[j..j+1].parse::<u32>().unwrap() + first_number).to_string());
                        break;
                    }
                }
                for j in i+2..number.len() - 1 {
                    if j < number.len() - 2 && number[j..j+3].parse::<u32>().is_ok() {
                        number.replace_range(j..j+2, &(number[j..j+3].parse::<u32>().unwrap() + second_number).to_string());
                        break;
                    } else if number[j..j+2].parse::<u32>().is_ok() {
                        number.replace_range(j..j+2, &(number[j..j+2].parse::<u32>().unwrap() + second_number).to_string());
                        break;
                    } else if number[j..j+1].parse::<u32>().is_ok() {
                        number.replace_range(j..j+1, &(number[j..j+1].parse::<u32>().unwrap() + second_number).to_string());
                        break;
                    }
                }

                continue 'main;
            }
        }

        // Check for regular number greater than 9
        for i in 0..number.len() - 1 {
            if number[i..i+2].parse::<u32>().is_ok() {
                let n = number[i..i+2].parse::<u32>().unwrap();

                if n % 2 == 0 {
                    number.replace_range(i..i+2, &format!("[{},{}]", n / 2, n / 2));
                } else {
                    number.replace_range(i..i+2, &format!("[{},{}]", n / 2, n / 2 + 1));
                }

                continue 'main;
            }
        }

        break;
    }

    serde_json::from_str::<Value>(&number).unwrap()
}
