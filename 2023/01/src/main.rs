fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();

    let mut result = 0;

    let written_digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut line = line.to_string();

        for (i, digit) in written_digits.iter().enumerate() {
            line = line.replace(i.to_string().as_str(), digit);
        }

        let mut first_digit = (usize::MAX, 0);
        let mut last_digit = (usize::MIN, 0);

        for (i, digit) in written_digits.iter().enumerate() {
            if let Some(idx) = line.find(digit) {
                if idx <= first_digit.0 {
                    first_digit = (idx, i);
                }
            }

            if let Some(idx) = line.rfind(digit) {
                if idx >= last_digit.0 {
                    last_digit = (idx, i);
                }
            }
        }

        result += first_digit.1 * 10 + last_digit.1;
    }

    println!("{result}");
}
