fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut i = 0;
    let mut elves = vec![0];

    for line in input.lines() {
        if line.is_empty() {
            elves.push(0);
            i += 1;
            continue;
        }

        let calories = line.parse::<u32>().unwrap();
        elves[i] += calories;
    }

    elves.sort();
    println!(
        "{}",
        elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]
    );
}
