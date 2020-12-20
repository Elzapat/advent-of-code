fn main() {

    let range = (245318, 765747);

    println!("Part 1: {}", part_1(&range));
    println!("Part 2: {}", part_2(&range));
}

fn part_1(range: &(i32, i32)) -> i32 {

    let mut nb_match = 0;

    for pwd in range.0 .. range.1 {
        let mut two_adjacent = false;
        let mut increasing = true;

        let mut last_digit = 0;

        for digit in pwd.to_string().chars() {
            let digit = digit.to_digit(10).unwrap(); 

            if digit == last_digit {
                two_adjacent = true;
            }

            if digit < last_digit {
                increasing = false;
                break;
            }

            last_digit = digit;
        }

        if two_adjacent && increasing {
            nb_match += 1;
        }
    }

    nb_match
}

fn part_2(range: &(i32, i32)) -> i32 {

    let mut nb_match = 0;

    for pwd in range.0 .. range.1 {
        let mut increasing = true;

        let mut last_digit = 0;
        let mut smallest_group = 10_000;
        let mut current_group = 1;

        for digit in pwd.to_string().chars() {
            let digit = digit.to_digit(10).unwrap(); 

            if digit == last_digit {
                current_group += 1;
            } else {
                if current_group < smallest_group && current_group != 1 {
                    smallest_group = current_group;
                }
                current_group = 1;
            }

            if digit < last_digit {
                increasing = false;
                break;
            }

            last_digit = digit;
        }

        if current_group < smallest_group && current_group != 1 {
            smallest_group = current_group;
        }

        if increasing && smallest_group == 2 {
            nb_match += 1;
        }
    }

    nb_match
}
