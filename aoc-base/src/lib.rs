use std::{env, fs};

pub fn read_input() -> String {
    let input_file = if env::var("AOC_EXAMPLE").is_ok() {
        "ex.txt"
    } else {
        "input.txt"
    };

    fs::read_to_string(input_file).unwrap()
}

pub fn read_input_grid() -> Vec<Vec<char>> {
    read_input()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
