fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            (parts.next().unwrap().to_owned(), parts.next().unwrap().parse::<i32>().unwrap())
        })
        .collect::<Vec<(String, i32)>>();

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

fn part1(input: Vec<(String, i32)>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for (command, x) in &input {
        match &command[..] {
            "forward" => horizontal += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => {},
        } 
    }

    horizontal * depth
}

fn part2(input: Vec<(String, i32)>) -> i32 {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for (command, x) in &input {
        match &command[..] {
            "forward" => {
                horizontal += x;
                depth += aim * x;
            },
            "down" => aim += x,
            "up" => aim -= x,
            _ => {}
        }
    }

    horizontal * depth
}
