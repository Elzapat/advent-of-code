fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut res = 0;

    for c in input.trim().chars() {
        res += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        };
    }

    println!("Part 1: {res}");

    res = 0;

    for (i, c) in input.trim().chars().enumerate() {
        res += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        };

        if res == -1 {
            println!("Part 2: {}", i + 1);
            break;
        }
    }
}
