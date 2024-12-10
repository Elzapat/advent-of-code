fn main() {
    let target_row = 2978;
    let target_col = 3083;

    let mut code: u128 = 20151125;
    let mut col = 1;
    let mut row = 1;
    let mut shift_length = 1;

    'find_code: loop {
        for _ in 0..shift_length {
            code *= 252533;
            code %= 33554393;

            row -= 1;
            col += 1;

            if col == target_col && row == target_row {
                break 'find_code;
            }
        }

        shift_length += 1;
        row = shift_length;
        col = 1;
    }

    println!("Part 1: {code}");
}
