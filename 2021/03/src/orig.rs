use std::time::SystemTime;

pub fn solve() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();

    let mut start = SystemTime::now();
    {
        let mut ones = vec![];
        let mut zeros = vec![];

        let mut epsilon = 0_u32;
        let mut gamma = 0_u32;

        for _ in 0..input[0].len() {
            ones.push(0);
            zeros.push(0);
        }

        for line in &input {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '1' => ones[i] += 1,
                    _ => zeros[i] += 1,
                }
            }
        }

        for i in 0..input[0].len() {
            if ones[i] < zeros[i] {
                // epsilon += (2.0_f32).powi(i as i32) as u32;
                epsilon += 1;
                // gamma <<= 1;
            } else {
                gamma += 1;
                // gamma += (2.0_f32).powi(i as i32) as u32;
                // epsilon <<= 1;
            }
            epsilon <<= 1;
            gamma <<= 1;
        }
        epsilon >>= 1;
        gamma >>= 1;

        println!("Part 1: {}", gamma * epsilon);
    }
    println!("Time part 1 orig: {}ms", start.elapsed().unwrap().as_micros() as f32 / 1000.0);

    start = SystemTime::now();
    {
        let mut o = input.clone();
        let mut co2 = input.clone();

        let mut ones_o = vec![];
        let mut zeros_o = vec![];

        let mut ones_co2 = vec![];
        let mut zeros_co2 = vec![];

        let mut i = 0;
        while o.len() > 1 || co2.len() > 1 {
            for _ in 0..o[0].len() {
                ones_o.push(0);
                zeros_o.push(0);
            }

            for _ in 0..co2[0].len() {
                ones_co2.push(0);
                zeros_co2.push(0);
            }

            for line in &o {
                for (j, c) in line.chars().enumerate() {
                    match c {
                        '1' => ones_o[j] += 1,
                        _ => zeros_o[j] += 1,
                    }
                }
            }

            for line in &co2 {
                for (j, c) in line.chars().enumerate() {
                    match c {
                        '1' => ones_co2[j] += 1,
                        _ => zeros_co2[j] += 1,
                    }
                }
            }

            if ones_o[i] >= zeros_o[i] {
                if o.len() > 1 { o.retain(|line| line.chars().nth(i).unwrap() == '1'); }
            } else {
                if o.len() > 1 { o.retain(|line| line.chars().nth(i).unwrap() == '0'); }
            }

            if ones_co2[i] >= zeros_co2[i] {
                if co2.len() > 1 { co2.retain(|line| line.chars().nth(i).unwrap() == '0') };
            } else {
                if co2.len() > 1 { co2.retain(|line| line.chars().nth(i).unwrap() == '1') };
            }

            i += 1;
            ones_o = vec![];
            zeros_o = vec![];
            ones_co2 = vec![];
            zeros_co2 = vec![];

        }
        println!("Part 2: {}", u32::from_str_radix(&o[0], 2).unwrap() * u32::from_str_radix(&co2[0], 2).unwrap());
    }
    println!("Time part 2 orig: {}ms", start.elapsed().unwrap().as_micros() as f32 / 1000.0);
}
