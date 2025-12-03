fn main() {
    let input = aoc_base::read_input();

    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for bank in input.lines() {
        result_p1 += joltage(bank, 2);
        result_p2 += joltage(bank, 12);
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}

fn joltage(bank: &str, mut digit: usize) -> u64 {
    let mut joltage = 0;
    let mut last_digit_idx = 0;

    while digit > 0 {
        let available_bank = &bank[last_digit_idx..bank.len() - digit + 1];
        let max = available_bank
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        let real_pos = available_bank
            .chars()
            .position(|c| c.to_string() == available_bank[max.0..max.0 + 1])
            .unwrap();

        joltage += max.1 as u64 * 10_u64.pow(digit as u32 - 1);
        last_digit_idx += real_pos + 1;

        digit -= 1;
    }

    joltage
}
