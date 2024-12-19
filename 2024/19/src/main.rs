fn main() {
    let lines = aoc_base::read_input()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let patterns = lines[0]
        .split(", ")
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let designs = lines[2..].to_vec();

    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for design in &designs {
        let nb_designs = make_design(patterns.clone(), design.clone());

        if nb_designs > 0 {
            result_p1 += 1;
        }

        result_p2 += nb_designs;
    }

    println!("Part 1: {}", result_p1);
    println!("Part 2: {}", result_p2);
}

#[memoize::memoize]
fn make_design(patterns: Vec<String>, design: String) -> u64 {
    if design.is_empty() {
        return 1;
    }

    let mut possible_ways = 0;

    for pattern in &patterns {
        if design.starts_with(pattern) {
            possible_ways += make_design(patterns.clone(), design[pattern.len()..].to_string());
        }
    }

    possible_ways
}
