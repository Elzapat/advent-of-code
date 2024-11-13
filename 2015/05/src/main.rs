fn main() {
    let input = aoc_base::read_input();
    let mut nice_strings = 0;

    for string in input.lines() {
        // Check forbidden sequences
        if string.contains("ab") || string.contains("cd") || string.contains("pq") || string.contains("xy") {
            continue;
        }

        // Check repeat letters
        let mut found_repeat = false;
        for i in 0..string.len() - 1 {
            let char = string.chars().nth(i).unwrap(); 
            let next_char = string.chars().nth(i + 1).unwrap(); 

            if char == next_char {
                found_repeat = true;
                break;
            }
        }
        if !found_repeat {
            continue;
        }

        // Check vowels
        let mut nb_vowels = 0;
        for c in string.chars() {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                nb_vowels += 1;
            }
        }
        if nb_vowels < 3 {
            continue;
        }

        nice_strings += 1;
    }

    println!("Part 1: {nice_strings}");

    nice_strings = 0;

    for string in input.lines() {
        if string.as_bytes().windows(3).filter(|w| w[0] == w[2]).count() == 0 {
            continue;
        }

        let mut found_pattern = false;
        for window in string.as_bytes().windows(2) {
            if string.matches(std::str::from_utf8(window).unwrap()).count() >= 2 {
                found_pattern = true;
                break;
            }
        }
        if !found_pattern {
            continue;
        }

        nice_strings += 1;
    }

    println!("Part 2: {nice_strings}");
}
