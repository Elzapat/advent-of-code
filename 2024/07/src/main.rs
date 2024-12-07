fn main() {
    let input = aoc_base::read_input();
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for line in input.lines() {
        let mut parts = line.split(": ");
        let target = parts.next().unwrap().parse::<u128>().unwrap();
        let numbers = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|n| n.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();

        if find_eq_result(&numbers, 0, target, 0, false) {
            result_p1 += target;
        }

        if find_eq_result(&numbers, 0, target, 0, true) {
            result_p2 += target;
        }
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}

fn find_eq_result(numbers: &Vec<u128>, i: usize, target: u128, result: u128, p2: bool) -> bool {
    if i > numbers.len() - 1 {
        return target == result;
    }

    find_eq_result(numbers, i + 1, target, result + numbers[i], p2)
        || find_eq_result(numbers, i + 1, target, result.max(1) * numbers[i], p2)
        || (p2
            && find_eq_result(
                numbers,
                i + 1,
                target,
                format!("{}{}", result, numbers[i]).parse::<u128>().unwrap(),
                p2,
            ))
}
