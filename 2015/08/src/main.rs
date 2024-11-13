fn main() {
    let input = aoc_base::read_input();
    let mut total_string_rep_size = 0;
    let mut total_in_mem_size = 0;

    for string in input.lines() {
        total_string_rep_size += string.len();

        let mut chars = string.chars().skip(1);
        let mut char = chars.next();

        while char != None { 
            if char == Some('\\') {
                char = chars.next();

                if char == Some('x') {
                    chars.next();
                    chars.next();
                }
            }

            total_in_mem_size += 1;
            char = chars.next();
        }

        total_in_mem_size -= 1;
    }

    println!("Part 1: {}", total_string_rep_size - total_in_mem_size);

    let mut total_encoded_string_size = 0;

    for string in input.lines() {
        total_encoded_string_size += 2;

        for c in string.chars() {
            total_encoded_string_size += match c {
                '\"' | '\\' => 2,
                _ => 1,
            }
        }
    }

    println!("Part 2: {}", total_encoded_string_size - total_string_rep_size);
}
