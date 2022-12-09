use std::{cmp::Ordering, collections::HashSet};

#[derive(Default, Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn distance(&self, other: Pos) -> i32 {
        (((self.x - other.x) as f32).powi(2) + ((self.y - other.y) as f32).powi(2)).sqrt() as i32
    }
}

fn clamp(n: i32) -> i32 {
    match n.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();

    println!("Part 1: {}", simulate(2, lines.clone()));
    println!("Part 2: {}", simulate(10, lines));
}

fn simulate(n_knots: u32, instructions: Vec<String>) -> usize {
    let mut visited_pos = HashSet::<Pos>::new();
    let mut knots = vec![];
    for _ in 0..n_knots {
        knots.push(Pos::default());
    }

    for line in &instructions {
        let dir = line.chars().next().unwrap();
        let n = line[2..].parse::<u32>().unwrap();

        for _ in 0..n {
            visited_pos.insert(knots[knots.len() - 1]);

            match dir {
                'R' => knots[0].x += 1,
                'L' => knots[0].x -= 1,
                'D' => knots[0].y += 1,
                'U' => knots[0].y -= 1,
                _ => unreachable!(),
            }

            for i in 1..knots.len() {
                if knots[i - 1].distance(knots[i]) > 1 {
                    knots[i].x += clamp(knots[i - 1].x - knots[i].x);
                    knots[i].y += clamp(knots[i - 1].y - knots[i].y);
                }
            }
        }
    }

    visited_pos.len()
}
