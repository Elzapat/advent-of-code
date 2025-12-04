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

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub r: usize,
    pub c: usize,
}

impl Point {
    pub fn new(r: usize, c: usize) -> Point {
        Point { r, c }
    }

    pub fn get_4_neighbours(&self, r_bound: usize, c_bound: usize) -> Vec<Point> {
        let mut n = vec![];

        for (dr, dc) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            if self.r as isize + dr >= 0
                && self.c as isize + dc >= 0
                && self.r as isize + dr < r_bound as isize
                && self.c as isize + dc < c_bound as isize
            {
                n.push(Point::new(
                    (self.r as isize + dr) as usize,
                    (self.c as isize + dc) as usize,
                ));
            }
        }

        n
    }

    pub fn get_4_neighbours_with_directions(
        &self,
        r_bound: usize,
        c_bound: usize,
        neighbours: Vec<(isize, isize)>,
    ) -> Vec<Point> {
        let mut n = vec![];

        for (dr, dc) in neighbours {
            if dr == 0 && dc == 0 {
                continue;
            }

            if self.r as isize + dr >= 0
                && self.c as isize + dc >= 0
                && self.r as isize + dr < r_bound as isize
                && self.c as isize + dc < c_bound as isize
            {
                n.push(Point::new(
                    (self.r as isize + dr) as usize,
                    (self.c as isize + dc) as usize,
                ));
            }
        }

        n
    }

    pub fn get_8_neighbours(&self, r_bound: usize, c_bound: usize) -> Vec<Point> {
        let mut n = vec![];

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }

                if self.r as isize + dr >= 0
                    && self.c as isize + dc >= 0
                    && self.r as isize + dr < r_bound as isize
                    && self.c as isize + dc < c_bound as isize
                {
                    n.push(Point::new(
                        (self.r as isize + dr) as usize,
                        (self.c as isize + dc) as usize,
                    ));
                }
            }
        }

        n
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        ((self.r as isize - other.r as isize).abs() + (self.c as isize - other.c as isize).abs())
            as usize
    }

    pub fn distance(&self, other: &Point) -> f32 {
        ((self.r as f32 - other.r as f32).powi(2) + (self.c as f32 - other.c as f32).powi(2)).sqrt()
    }
}
