fn main() {
    let input = aoc_base::read_input();
    let mut num = 0;

    loop {
        let hash = format!("{}{}", input.trim(), num);
        let digest = md5::compute(hash);

        if format!("{:x}", digest).starts_with("000000") {
            println!("Part 1: {}", num);
            break;
        }

        num += 1;
    }
}
