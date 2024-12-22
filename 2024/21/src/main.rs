use aoc_base::Point;
use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};

lazy_static! {
    static ref numpad_pos: HashMap<char, Point> = HashMap::from([
        ('7', Point::new(0, 0)),
        ('8', Point::new(0, 1)),
        ('9', Point::new(0, 2)),
        ('4', Point::new(1, 0)),
        ('5', Point::new(1, 1)),
        ('6', Point::new(1, 2)),
        ('1', Point::new(2, 0)),
        ('2', Point::new(2, 1)),
        ('3', Point::new(2, 2)),
        ('0', Point::new(3, 1)),
        ('A', Point::new(3, 2)),
    ]);
    static ref keypad_pos: HashMap<char, Point> = HashMap::from([
        ('^', Point::new(0, 1)),
        ('A', Point::new(0, 2)),
        ('<', Point::new(1, 0)),
        ('v', Point::new(1, 1)),
        ('>', Point::new(1, 2)),
    ]);
}

fn main() {
    let codes = aoc_base::read_input()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    println!("Part 1: {}", input_codes(codes.clone(), 2));
    // 179998901743668
    println!("Part 2: {}", input_codes(codes.clone(), 25));
}

fn input_codes(codes: Vec<String>, nb_keypads: usize) -> usize {
    let mut result = 0;
    let mut numpad_robot_pos = numpad_pos[&'A'];

    for code in &codes {
        let mut code_result = 0;

        for digit in code.chars() {
            let target = numpad_pos[&digit];
            code_result += compute_least_keypress(numpad_robot_pos, target, nb_keypads);
            numpad_robot_pos = target;
        }

        result += code_result * code[..3].parse::<usize>().unwrap();
    }

    result
}

fn compute_least_keypress(pos: Point, target: Point, nb_keypads: usize) -> usize {
    let mut result = usize::MAX;

    let mut queue = VecDeque::new();
    queue.push_back((pos, "".to_string()));

    while !queue.is_empty() {
        let (cur_pos, presses) = queue.pop_front().unwrap();

        if cur_pos == target {
            let best = move_robot(format!("{presses}A"), 0, nb_keypads);
            result = result.min(best);
            continue;
        }

        if !numpad_pos.values().any(|p| *p == cur_pos) {
            continue;
        } else {
            if cur_pos.r < target.r {
                queue.push_back((Point::new(cur_pos.r + 1, cur_pos.c), format!("{presses}v")));
            } else if cur_pos.r > target.r {
                queue.push_back((Point::new(cur_pos.r - 1, cur_pos.c), format!("{presses}^")));
            }

            if cur_pos.c < target.c {
                queue.push_back((Point::new(cur_pos.r, cur_pos.c + 1), format!("{presses}>")));
            } else if cur_pos.c > target.c {
                queue.push_back((Point::new(cur_pos.r, cur_pos.c - 1), format!("{presses}<")));
            }
        }
    }

    result
}

fn move_robot(presses: String, robot_idx: usize, nb_keypads: usize) -> usize {
    if robot_idx == nb_keypads {
        return presses.len();
    }

    let mut result = 0;
    let mut robot_pos = keypad_pos[&'A'];

    for press in presses.chars() {
        let target = keypad_pos[&press];
        result += compute_best_keypad_presses(robot_pos, target, robot_idx, nb_keypads);
        robot_pos = target;
    }

    result
}

#[memoize::memoize]
fn compute_best_keypad_presses(
    pos: Point,
    target: Point,
    robot_idx: usize,
    nb_keypads: usize,
) -> usize {
    let mut result = usize::MAX;

    let mut queue = VecDeque::new();
    queue.push_back((pos, "".to_string()));

    while !queue.is_empty() {
        let (cur_pos, presses) = queue.pop_front().unwrap();

        if cur_pos == target {
            let best = move_robot(format!("{presses}A"), robot_idx + 1, nb_keypads);
            result = result.min(best);
            continue;
        }

        if !keypad_pos.values().any(|p| *p == cur_pos) {
            continue;
        } else {
            if cur_pos.r < target.r {
                queue.push_back((Point::new(cur_pos.r + 1, cur_pos.c), format!("{presses}v")));
            } else if cur_pos.r > target.r {
                queue.push_back((Point::new(cur_pos.r - 1, cur_pos.c), format!("{presses}^")));
            }

            if cur_pos.c < target.c {
                queue.push_back((Point::new(cur_pos.r, cur_pos.c + 1), format!("{presses}>")));
            } else if cur_pos.c > target.c {
                queue.push_back((Point::new(cur_pos.r, cur_pos.c - 1), format!("{presses}<")));
            }
        }
    }

    result
}
