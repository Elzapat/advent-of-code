fn main() {
    let input = aoc_base::read_input();

    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for range in input.split(',') {
        let mut parts = range.split('-');
        let start = parts.next().unwrap().parse::<u64>().unwrap();
        let end = parts.next().unwrap().parse::<u64>().unwrap();

        for n in start..=end {
            let n_str = n.to_string();
            let split = n_str.split_at(n_str.len() / 2);

            if split.0 == split.1 {
                result_p1 += n;
            }

            for window in 1..=(n_str.len() / 2) {
                if n_str.len() % window != 0 {
                    continue;
                }

                let mut invalid = true;
                let pattern = n_str[0..window].to_string();

                for start in (window..=n_str.len() - window).step_by(window) {
                    if pattern != n_str[start..start + window] {
                        invalid = false;
                        break;
                    }
                }

                if invalid {
                    result_p2 += n;
                    break;
                }
            }
        }
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}
