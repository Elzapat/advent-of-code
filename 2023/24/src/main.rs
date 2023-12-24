use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Hailstone {
    p: Vector2<f64>,
    v: Vector2<f64>,
}

fn main() {
    let input = aoc_base::read_input();

    let mut hailstones = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(" @ ");
        let mut pos = parts.next().unwrap().split(", ");
        let mut vel = parts.next().unwrap().split(", ");

        hailstones.push(Hailstone {
            p: Vector2::new(
                pos.next().unwrap().trim().parse::<f64>().unwrap(),
                pos.next().unwrap().trim().parse::<f64>().unwrap(),
            ),
            v: Vector2::new(
                vel.next().unwrap().trim().parse::<f64>().unwrap(),
                vel.next().unwrap().trim().parse::<f64>().unwrap(),
            ),
        });
    }

    hailstones.push(*hailstones.first().unwrap());

    let mut result = 0;
    let mut seen = Vec::new();

    for (h1, h2) in hailstones.iter().tuple_combinations() {
        if seen.contains(&(h1, h2)) {
            continue;
        }

        seen.push((h1, h2));
        seen.push((h2, h1));

        let equation_matrix = Matrix2::new(h1.v.x, -h2.v.x, h1.v.y, -h2.v.y);
        let equation_vector = h2.p - h1.p;

        if let Some(sol_t) = equation_matrix
            .try_inverse()
            .map(|inv| inv * equation_vector)
        {
            if sol_t.x < 0.0 || sol_t.y < 0.0 {
                continue;
            }

            let pos = h1.v * sol_t.x + h1.p;

            if (200000000000000.0..=400000000000000.0).contains(&pos.x)
                && (200000000000000.0..=400000000000000.0).contains(&pos.y)
            {
                // if (7.0..=27.0).contains(&pos.x) && (7.0..=27.0).contains(&pos.y) {
                result += 1;
            }
        }
    }

    println!("Part 1: {result}");
}
