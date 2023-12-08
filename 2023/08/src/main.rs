use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

fn main() {
    let input = aoc_base::read_input();
    let mut lines = input.lines();

    let instructions = lines.next().unwrap();
    lines.next().unwrap();

    let mut nodes = HashMap::<String, Node>::new();

    for line in lines {
        let mut split = line.split("=");
        let origin = split.next().unwrap().trim();
        let left_right = split
            .next()
            .unwrap()
            .trim()
            .replace("(", "")
            .replace(")", "");
        let split = left_right.split(", ").collect::<Vec<&str>>();

        nodes.insert(
            origin.to_string(),
            Node {
                left: split[0].to_string(),
                right: split[1].to_string(),
            },
        );
    }

    let mut current_node = "AAA".to_string();
    let mut result_p1 = 0;
    let mut i = 0;

    while current_node != "ZZZ".to_string() {
        let instruction = instructions.chars().nth(i).unwrap();

        println!("{current_node}");
        current_node = match instruction {
            'R' => nodes[&current_node].right.clone(),
            'L' => nodes[&current_node].left.clone(),
            _ => unreachable!(),
        };

        result_p1 += 1;
        i = (i + 1) % instructions.len();
    }

    let mut current_nodes = nodes
        .iter()
        .map(|(k, _)| k.clone())
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<String>>();
    let mut result_p2 = 0;
    let mut i = 0;

    while !current_nodes.iter().all(|k| k.ends_with("Z")) {
        let instruction = instructions.chars().nth(i).unwrap();

        for node in &mut current_nodes {
            *node = match instruction {
                'L' => nodes[node].left.clone(),
                'R' => nodes[node].right.clone(),
                _ => unreachable!(),
            }
        }

        result_p2 += 1;
        i = (i + 1) % instructions.len();
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}
