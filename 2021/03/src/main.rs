mod orig;

use std::time::SystemTime;

const N: usize = 12;

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
        .unwrap()
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u32>>();

    orig::solve();
    println!("--------------------------------------------");

    let mut start = SystemTime::now();
    println!("Part 1: {}", part_1(input.clone()));
    println!("Time part 1: {}ms", start.elapsed().unwrap().as_micros() as f32 / 1000.0);

    start = SystemTime::now();
    println!("Part 2: {}", part_2(input));
    println!("Time part 2: {}ms", start.elapsed().unwrap().as_micros() as f32 / 1000.0);
}

fn part_1(input: Vec<u32>) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    let mut ones = [0; N];

    for x in &input {
        for i in (0..N).rev() {
            ones[N - i - 1] += (x & (1 << i) > 0) as u32;
        }
    }

    for n in ones {
        if n >= (input.len() / 2) as u32 {
            gamma += 1;
        } else {
            epsilon += 1;
        }

        gamma <<= 1;
        epsilon <<= 1;
    }
    gamma >>= 1;
    epsilon >>= 1;

    gamma * epsilon
}

fn part_2(input: Vec<u32>) -> u32 {
    let mut o = input.clone();
    let mut co2 = input;

    let mut i = 0;
    while o.len() > 1 {
        let mut ones = 0;

        for x in &o {
            ones += (*x & (1 << (N - 1 - i)) > 0) as u32;
        }

        if ones >= o.len() as u32 - ones {
            o.retain(|x| (*x & (1 << (N - 1 - i))) > 0);
        } else {
            o.retain(|x| *x & (1 << (N - 1 - i)) == 0);
        }

        i += 1;
    }

    let mut i = 0;
    while co2.len() > 1 {
        let mut ones = 0;

        for x in &co2 {
            ones += (*x & (1 << (N - 1 - i)) > 0) as u32;
        }

        if ones >= co2.len() as u32 - ones {
            co2.retain(|x| *x & (1 << (N - 1 - i)) == 0);
        } else {
            co2.retain(|x| *x & (1 << (N - 1 - i)) > 0);
        }

        i += 1;
    }

    o[0] * co2[0]
}
