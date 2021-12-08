use std::fs;

fn main() {

    let mut intcode = fs::read_to_string("input.txt").expect("Reading file error");
    intcode.pop();

    let mut codes = vec![];

    for code in intcode.split(',') {
        codes.push(code.parse::<i32>().unwrap());
    }
    let mut codes_clone = codes.clone();
    codes_clone[1] = 12;
    codes_clone[2] = 2;

    println!("Part 1: {}", part_1(&mut codes_clone));
    println!("Part 2: {}", part_2(&mut codes));
}

fn part_1(codes: &mut Vec<i32>) -> i32 {

    let mut i = 0;
    loop {
        match codes[i] {
            1 => {
                let first_pos = codes[i + 1] as usize;
                let second_pos = codes[i + 2] as usize;
                let third_pos = codes[i + 3] as usize;
                let sum = codes[first_pos] + codes[second_pos];
                codes[third_pos] = sum;
            },
            2 => {
                let first_pos = codes[i + 1] as usize;
                let second_pos = codes[i + 2] as usize;
                let third_pos = codes[i + 3] as usize;
                let product = codes[first_pos] * codes[second_pos];
                codes[third_pos] = product;
            },
            99 => break,
            _ => panic!("Problem"),
        }

        i += 4;
    }

    codes[0]
}

fn part_2(codes: &mut Vec<i32>) -> i32 {

    let original_codes = codes.clone();

    for i in 0..100 {
        for j in 0..100 {
            codes[1] = i;
            codes[2] = j;
            match part_1(codes) {
                19690720 => return 100 * i + j,
                _ => *codes = original_codes.clone(),
            }
        }
    }

    0
}
