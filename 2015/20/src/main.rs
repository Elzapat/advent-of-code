fn main() {
    let input = 34000000;

    let mut current_house = 1;
    while compute_house_presents_p1(current_house) < input {
        current_house += 1;
    }

    println!("Part 1: {}", current_house);

    let mut current_house_p2 = 1;
    while compute_house_presents_p2(current_house_p2) < input {
        current_house_p2 += 1;
    }

    println!("Part 2: {}", current_house_p2);
}

fn compute_house_presents_p1(house_nb: u64) -> u64 {
    let mut divisors = divisors::get_divisors(house_nb);
    divisors.push(1);
    divisors.push(house_nb);

    divisors.iter().map(|d| 10 * d).sum::<u64>()
}

fn compute_house_presents_p2(house_nb: u64) -> u64 {
    let mut divisors = divisors::get_divisors(house_nb);
    divisors.push(1);
    divisors.push(house_nb);

    divisors = divisors.into_iter().filter(|d| d * 50 > house_nb).collect();

    divisors.iter().map(|d| 11 * d).sum::<u64>()
}
