fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum = 0;
    let mut i = 0;

    // Part 2
    while i < lines.len() {
        for c in lines[i].chars() {
            if lines[i + 1].contains(c) && lines[i + 2].contains(c) {
                sum += if c.is_lowercase() {
                    c as u8 - b'a' + 1
                } else {
                    c as u8 - b'A' + 1 + 26
                } as u32;

                i += 3;
                break;
            }
        }
    }

    // Part 1
    /*
    for line in input.lines() {
        let size = line.len() / 2;
        let first = &line[..size];
        let second = &line[size..];
        let mut done_letters = std::collections::HashSet::new();

        for c1 in first.chars() {
            for c2 in second.chars() {
                if c1 == c2 && !done_letters.contains(&c1) {
                    done_letters.insert(c1);
                    sum += if c1.is_lowercase() {
                        c1 as u8 - b'a' + 1
                    } else {
                        c1 as u8 - b'A' + 1 + 26
                    } as u32;
                }
            }
        }
    }
    */

    println!("{sum}");
}
