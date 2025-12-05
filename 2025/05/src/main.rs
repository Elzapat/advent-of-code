fn main() {
    let input = aoc_base::read_input();

    let mut processing_ranges = true;
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut result_p1 = 0;
    let mut result_p2 = 0_u64;

    'main: for line in input.lines() {
        if line.is_empty() {
            processing_ranges = false;
            continue;
        }

        if processing_ranges {
            let mut parts = line.split('-');
            ranges.push((
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            ));
            continue;
        }

        let id = line.parse::<u64>().unwrap();
        for range in &ranges {
            if id >= range.0 && id <= range.1 {
                result_p1 += 1;
                continue 'main;
            }
        }
    }

    ranges.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    ranges.reverse();

    while let Some(r1) = ranges.pop() {
        if let Some(r2) = ranges.pop() {
            if r1.1 >= r2.0 {
                ranges.push((r1.0, r1.1.max(r2.1)));
            } else {
                ranges.push(r2);
                result_p2 += r1.1 - r1.0 + 1;
            }
        } else {
            result_p2 += r1.1 - r1.0 + 1;
        }
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}
