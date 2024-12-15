use aoc_base::Point;

fn main() {
    let lines = aoc_base::read_input()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut map = vec![];
    let mut directions = "".to_string();

    let mut map_input = true;

    for line in lines {
        if line.is_empty() {
            map_input = !map_input;
        }

        if map_input {
            map.push(line.chars().collect::<Vec<char>>());
        } else {
            directions.push_str(&line);
        }
    }

    let og_map = map.clone();

    let mut robot_pos = Point::new(0, 0);

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == '@' {
                robot_pos = Point::new(r, c);
            }
        }
    }

    for direction in directions.chars() {
        let dir = match direction {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => unreachable!(),
        };

        let move_pos = Point::new(
            (robot_pos.r as isize + dir.0) as usize,
            (robot_pos.c as isize + dir.1) as usize,
        );
        match map[move_pos.r][move_pos.c] {
            '.' => {
                map[move_pos.r][move_pos.c] = '@';
                map[robot_pos.r][robot_pos.c] = '.';
                robot_pos = move_pos;
            }
            'O' => {
                if move_box(&mut map, move_pos, dir) {
                    map[move_pos.r][move_pos.c] = '@';
                    map[robot_pos.r][robot_pos.c] = '.';
                    robot_pos = move_pos;
                }
            }
            _ => {}
        }
    }

    let mut result_p1 = 0;

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == 'O' {
                result_p1 += 100 * r + c;
            }
        }
    }

    println!("Part 1: {result_p1}");

    let mut map2 = vec![];

    for row in &og_map {
        let mut map2_row = vec![];

        for cell in row {
            match cell {
                'O' => {
                    map2_row.push('[');
                    map2_row.push(']');
                }
                '@' => {
                    map2_row.push('@');
                    map2_row.push('.');
                }
                _ => {
                    map2_row.push(*cell);
                    map2_row.push(*cell);
                }
            }
        }

        map2.push(map2_row);
    }

    let mut robot_pos = Point::new(0, 0);

    for r in 0..map2.len() {
        for c in 0..map2[r].len() {
            if map2[r][c] == '@' {
                robot_pos = Point::new(r, c);
            }
        }
    }

    for direction in directions.chars() {
        let dir = match direction {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => unreachable!(),
        };

        let move_pos = Point::new(
            (robot_pos.r as isize + dir.0) as usize,
            (robot_pos.c as isize + dir.1) as usize,
        );
        match map2[move_pos.r][move_pos.c] {
            '.' => {
                map2[move_pos.r][move_pos.c] = '@';
                map2[robot_pos.r][robot_pos.c] = '.';
                robot_pos = move_pos;
            }
            '[' | ']' => {
                let mut map2_clone = map2.clone();
                if move_box_p2(&mut map2_clone, move_pos, dir) {
                    map2 = map2_clone;
                    map2[move_pos.r][move_pos.c] = '@';
                    map2[robot_pos.r][robot_pos.c] = '.';
                    robot_pos = move_pos;
                }
            }
            _ => {}
        }
    }

    let mut result_p2 = 0;

    for r in 0..map2.len() {
        for c in 0..map2[r].len() {
            if map2[r][c] == '[' {
                result_p2 += 100 * r + c;
            }
        }
    }

    println!("Part 2: {result_p2}");
}

fn move_box(map: &mut Vec<Vec<char>>, box_pos: Point, dir: (isize, isize)) -> bool {
    if map[box_pos.r][box_pos.c] != 'O' {
        return false;
    }

    let push_pos = (box_pos.r as isize + dir.0, box_pos.c as isize + dir.1);

    if push_pos.0 < 0
        || push_pos.1 < 0
        || push_pos.0 > map.len() as isize - 1
        || push_pos.1 > map[0].len() as isize - 1
    {
        return false;
    }

    let push_pos = Point::new(push_pos.0 as usize, push_pos.1 as usize);

    let box_can_move = match map[push_pos.r][push_pos.c] {
        '#' => false,
        'O' => move_box(map, push_pos, dir),
        '.' => true,
        _ => unreachable!(),
    };

    if box_can_move {
        map[push_pos.r][push_pos.c] = 'O';
        map[box_pos.r][box_pos.c] = '.';
    }

    box_can_move
}

fn move_box_p2(map: &mut Vec<Vec<char>>, box_pos: Point, dir: (isize, isize)) -> bool {
    if map[box_pos.r][box_pos.c] != '[' && map[box_pos.r][box_pos.c] != ']' {
        return false;
    }

    let other_side = match map[box_pos.r][box_pos.c] {
        '[' => Point::new(box_pos.r, box_pos.c + 1),
        ']' => Point::new(box_pos.r, box_pos.c - 1),
        _ => unreachable!(),
    };

    let push_pos = (box_pos.r as isize + dir.0, box_pos.c as isize + dir.1);
    let push_pos_side = (other_side.r as isize + dir.0, other_side.c as isize + dir.1);

    let push_pos = Point::new(push_pos.0 as usize, push_pos.1 as usize);
    let push_pos_side = Point::new(push_pos_side.0 as usize, push_pos_side.1 as usize);

    let box_can_move = match map[push_pos.r][push_pos.c] {
        '#' => false,
        '[' | ']' => push_pos == other_side || move_box_p2(map, push_pos, dir),
        '.' => true,
        _ => unreachable!(),
    };

    let side_can_move = match map[push_pos_side.r][push_pos_side.c] {
        '#' => false,
        '[' | ']' => move_box_p2(map, push_pos_side, dir),
        '.' => true,
        _ => unreachable!(),
    };

    if box_can_move && side_can_move {
        map[push_pos_side.r][push_pos_side.c] = map[other_side.r][other_side.c];
        map[other_side.r][other_side.c] = '.';

        map[push_pos.r][push_pos.c] = map[box_pos.r][box_pos.c];
        map[box_pos.r][box_pos.c] = '.';
    }

    box_can_move && side_can_move
}
