use std::collections::HashMap;

fn hash(string: &str) -> u32 {
    let mut current_value = 0;

    for c in string.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}
fn main() {
    let input = aoc_base::read_input();
    let strings = input.trim().split(',').collect::<Vec<&str>>();

    println!("Part 1: {}", strings.iter().map(|s| hash(s)).sum::<u32>());

    let mut boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    for s in strings {
        let dash = s.chars().position(|c| c == '-');
        let equal = s.chars().position(|c| c == '=');

        if let Some(op_idx) = dash {
            let label = &s[0..op_idx];
            let box_number = hash(label);

            if boxes.contains_key(&box_number) {
                boxes.insert(
                    box_number,
                    boxes[&box_number]
                        .iter()
                        .filter(|(l, _)| l != &label)
                        .cloned()
                        .collect(),
                );
            }
        } else if let Some(op_idx) = equal {
            let label = &s[0..op_idx];
            let box_number = hash(label);
            let focal_length = s.chars().last().unwrap().to_digit(10).unwrap();

            let mut label_found = false;

            'boxes_loop: for (_, lenses) in &mut boxes {
                for lens in lenses {
                    if lens.0 == label {
                        lens.1 = focal_length;
                        label_found = true;
                        break 'boxes_loop;
                    }
                }
            }

            if !label_found {
                boxes
                    .entry(box_number)
                    .or_insert(Vec::new())
                    .push((label, focal_length));
            }
        }
    }

    println!(
        "Part 2: {}",
        boxes
            .iter()
            .map(|(box_number, lenses)| lenses
                .iter()
                .enumerate()
                .map(|(i, (_, focal))| (1 + box_number) * (i as u32 + 1) * (focal))
                .sum::<u32>())
            .sum::<u32>()
    );
}
