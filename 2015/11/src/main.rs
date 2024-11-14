fn main() {
    let input = aoc_base::read_input().trim().to_string();
    let mut password = increment_password(&input);

    while !check_password(&password) {
        password = increment_password(&password);
    }

    println!("Part 1: {password}");

    password = increment_password(&password);
    while !check_password(&password) {
        password = increment_password(&password);
    }

    println!("Part 2: {password}");
}

fn increment_password(password: &String) -> String {
    let mut output = "".to_string();
    let mut carry = true;

    for c in password.chars().rev() {
        let mut new_char = c;

        if carry {
            carry = false;

            if c == 'z' {
                carry = true;
                new_char = 'a';
            } else {
                new_char = (new_char as u8 + 1) as char;
            }
        }

        output.insert(0, new_char);
    }

    output
}

fn check_password(password: &String) -> bool {
    if password.len() != 8 || *password != password.to_lowercase() {
        return false;
    }

    let mut found_sequence = false;
    for i in 0..password.len() - 2 {
        let mut chars = password.chars().skip(i);
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();
        let c3 = chars.next().unwrap();

        if c1 as i16 - c2 as i16 == -1 && c1 as i16 - c3 as i16 == -2 {
            found_sequence = true;
            break;
        } 
    }
    if !found_sequence {
        return false;
    }

    if password.contains('i') || password.contains('o') || password.contains('l') {
        return false;
    }

    let mut nb_sequences = 0;
    let mut i = 0;
    while i < password.len() - 1 {
        let mut chars = password.chars().skip(i);
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();

        if c1 == c2 {
            nb_sequences += 1;
            i += 1;
        }

        i += 1;
    }
    if nb_sequences < 2 {
        return false;
    }

    true
}
