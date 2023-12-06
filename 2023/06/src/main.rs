fn compute_result(races: Vec<(u64, u64)>) -> u64 {
    let mut result = 1;

    for (race_time, race_record) in &races {
        let mut valid_results = 0;

        for button_held in 1..*race_time {
            let race_result = button_held * (*race_time - button_held);

            if race_result > *race_record {
                valid_results += 1;
            }
        }

        result *= valid_results;
    }

    return result;
}
fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();

    let first = input.lines().nth(0).unwrap();
    let last = input.lines().nth(1).unwrap();

    let races = first
        .split_whitespace()
        .skip(1)
        .zip(last.split_whitespace().skip(1))
        .map(|(time, distance)| {
            (
                time.parse::<u64>().unwrap(),
                distance.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<(u64, u64)>>();

    let races_p2 = first
        .split(':')
        .skip(1)
        .zip(last.split(':').skip(1))
        .map(|(time, distance)| {
            (
                time.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap(),
                distance
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap(),
            )
        })
        .collect::<Vec<(u64, u64)>>();

    println!("Part 1: {}", compute_result(races));
    println!("Part 2: {}", compute_result(races_p2));
}
