use std::{env, fs};

pub fn read_input() -> String {
    let input_file = if env::var("AOC_EX").is_ok() {
        "ex.txt"
    } else {
        "input.txt"
    };

    fs::read_to_string(input_file).unwrap().trim().to_string()
}

pub fn read_input_grid() -> Vec<Vec<char>> {
    read_input()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn read_input_grid_char() -> Vec<Vec<char>> {
    read_input()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn read_input_grid_i64() -> Vec<Vec<i64>> {
    read_input()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

pub fn read_input_grid_u64() -> Vec<Vec<u64>> {
    read_input()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}

pub fn read_input_grid_bool(char_true: char) -> Vec<Vec<bool>> {
    read_input()
        .lines()
        .map(|l| l.chars().map(|c| c == char_true).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>()
}
