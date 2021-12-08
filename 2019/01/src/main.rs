use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {

    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut input = vec![];
    for l in lines {
        if let Ok(l) = l {
            input.push(l.parse::<i32>().unwrap());
        }
    }

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Vec<i32>) -> f32 {

    let mut sum = 0.0;

    for mass in input.iter() {
        sum += (*mass as f32 / 3 as f32).floor() - 2 as f32;
    }

    sum
}

fn part_2(input: &Vec<i32>) -> f32 {

    let mut sum = 0.0;

    for mass in input.iter() {
        let mut fuel = *mass as f32;
        loop {
            fuel = (fuel / 3.0).floor() - 2.0;

            if fuel <= 0.0 {
                break;
            }

            sum += fuel;
        }
    }

    sum
}
