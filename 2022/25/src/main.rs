fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();

    let mut sum_decimal = 0;
    for line in input.lines() {
        sum_decimal += snafu_to_decimal(line);
    }

    println!("{sum_decimal}");
    println!("{}", decimal_to_snafu(1747));
    println!("{}", decimal_to_snafu(198));
    println!("{}", decimal_to_snafu(1257));
    // 2-=0-=-2=111-=-22102
    println!("Part 1: {}", decimal_to_snafu(sum_decimal));
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut res = 0;

    for (i, c) in snafu.chars().rev().enumerate() {
        let converted = match c {
            '-' => -1,
            '=' => -2,
            c => c.to_digit(10).unwrap() as i64,
        };

        res += converted * 5_i64.pow(i as u32) as i64;
    }

    res
}

fn decimal_to_snafu(mut decimal: i64) -> String {
    let mut res = vec![];

    while decimal > 0 {
        // dbg!(decimal as u32 % 5);
        let converted = match decimal as u64 % 5 {
            4 => '-',
            3 => '=',
            d => char::from_digit(d as u32, 10).unwrap(),
        };
        // dbg!(converted);

        decimal /= 5;
        // dbg!(decimal);
        if converted == '-' || converted == '=' {
            decimal += 1;
        }

        res.insert(0, converted);
    }

    res.iter().collect()
}
