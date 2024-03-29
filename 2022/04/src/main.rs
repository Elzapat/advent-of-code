fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut result = 0;

    for line in input.lines() {
        let idx = line.find('-').unwrap();
        let idx2 = line.find(',').unwrap();
        let first_elf =
            line[..idx].parse::<u32>().unwrap()..=line[idx + 1..idx2].parse::<u32>().unwrap();

        let idx3 = line[idx2..].find('-').unwrap() + idx2;
        let second_elf =
            line[idx2 + 1..idx3].parse::<u32>().unwrap()..=line[idx3 + 1..].parse::<u32>().unwrap();

        let mut contains = false;
        for n in first_elf.clone() {
            if second_elf.contains(&n) {
                contains = true;
                break;
            }
        }

        if !contains {
            contains = false;
            for n in second_elf {
                if first_elf.contains(&n) {
                    contains = true;
                    break;
                }
            }
        }

        if contains {
            result += 1;
        }
    }

    println!("{result}");
    solve_with_regex();
}

fn solve_with_regex() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut result = 0;
    let re = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
            if captures[1].parse::<u32>().unwrap() <= captures[4].parse::<u32>().unwrap()
                && captures[2].parse::<u32>().unwrap() >= captures[3].parse::<u32>().unwrap()
            {
                result += 1;
            }
        }
    }

    println!("{result}");
}
