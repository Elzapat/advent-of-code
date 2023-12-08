use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

fn find_nb_instructions(
    nodes: &HashMap<String, Node>,
    instructions: &str,
    start_node: &str,
) -> u64 {
    let mut current_node = start_node.to_string();
    let mut result = 0;
    let mut i = 0;

    while !current_node.ends_with('Z') {
        let instruction = instructions.chars().nth(i).unwrap();

        current_node = match instruction {
            'R' => nodes[&current_node].right.clone(),
            'L' => nodes[&current_node].left.clone(),
            _ => unreachable!(),
        };

        result += 1;
        i = (i + 1) % instructions.len();
    }

    result
}

fn main() {
    let input = aoc_base::read_input();
    let mut lines = input.lines();

    let instructions = lines.next().unwrap();
    lines.next().unwrap();

    let mut nodes = HashMap::<String, Node>::new();

    for line in lines {
        let mut split = line.split('=');
        let origin = split.next().unwrap().trim();
        let left_right = split.next().unwrap().trim().replace(['(', ')'], "");
        let split = left_right.split(", ").collect::<Vec<&str>>();

        nodes.insert(
            origin.to_string(),
            Node {
                left: split[0].to_string(),
                right: split[1].to_string(),
            },
        );
    }

    println!(
        "Part 1: {}",
        find_nb_instructions(&nodes, instructions, "AAA")
    );

    println!(
        "Part 2: {}",
        nodes
            .keys()
            .filter(|k| k.ends_with('A'))
            .fold(1, |result, node| {
                lcm(result, find_nb_instructions(&nodes, instructions, node))
            })
    );
}
