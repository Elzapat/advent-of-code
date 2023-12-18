fn compute_total_area(mut holes: Vec<(i64, i64)>) -> i64 {
    holes.push((0, 0));

    let area = holes
        .iter()
        .zip(holes.iter().skip(1))
        .map(|(h1, h2)| h1.0 * h2.1 - h2.0 * h1.1)
        .sum::<i64>()
        / 2;

    holes.pop();

    let interior_points = -(-area + (holes.len() as i64 / 2) - 1);

    interior_points + holes.len() as i64
}

fn main() {
    let input = aoc_base::read_input();

    let mut holes_p1 = Vec::new();
    let mut holes_p2 = Vec::new();

    let mut current_pos_p1 = (0, 0);
    let mut current_pos_p2 = (0, 0);

    for line in input.lines() {
        let mut parts = line.split(' ');

        let dir = parts.next().unwrap();
        let n = parts.next().unwrap().parse::<i64>().unwrap();
        let color = parts.next().unwrap().replace(['(', ')', '#'], "");

        for _ in 0..n {
            match dir {
                "U" => current_pos_p1.1 -= 1,
                "D" => current_pos_p1.1 += 1,
                "R" => current_pos_p1.0 += 1,
                "L" => current_pos_p1.0 -= 1,
                _ => unreachable!(),
            }

            holes_p1.push(current_pos_p1);
        }

        let dir = color.chars().last().unwrap().to_digit(16).unwrap() as i64;
        let n = i64::from_str_radix(&color[0..5], 16).unwrap();

        for _ in 0..n {
            match dir {
                0 => current_pos_p2.0 += 1,
                1 => current_pos_p2.1 += 1,
                2 => current_pos_p2.0 -= 1,
                3 => current_pos_p2.1 -= 1,
                _ => unreachable!(),
            }

            holes_p2.push(current_pos_p2);
        }
    }

    println!("Part 1: {}", compute_total_area(holes_p1));
    println!("Part 2: {}", compute_total_area(holes_p2));
}
