use std::fs;

struct Instr {
    distance: i32,
    direction: char,
}

fn main() {

    let wires_s = fs::read_to_string("3.txt").expect("Error while reading file!");

    let mut first_wire = vec![];
    let mut second_wire = vec![];

    for (i, wire) in wires_s.split('\n').enumerate() {
        for instr in wire.split(',') {
            match i {
                0 => {
                    first_wire.push(Instr {
                        direction: instr.chars().next().unwrap(),
                        distance: instr[1..].parse().unwrap(),
                    });
                },
                1 => {
                    second_wire.push(Instr {
                        direction: instr.chars().next().unwrap(),
                        distance: instr[1..].parse().unwrap(),
                    });
                },
                _ => panic!("Problem"),
            }
        }
    }

    println!("Part 1: {}", part_1(&mut first_wire, &mut second_wire));
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {

    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn compute_distance(instruction: &Instr) -> (i32, i32) {

    match instruction.direction {
        'L' => return (-instruction.distance, 0),
        'R' => return (instruction.distance, 0),
        'D' => return (0, instruction.distance),
        'U' => return (0, -instruction.distance),
        _ => panic!("Invalid direction"),
    }
}

fn part_1(first_wire: &mut Vec<Instr>, second_wire: &mut Vec<Instr>) -> i32 {

    let mut intersections = vec![];
    let mut first_wire_pos = vec![(0, 0)];
    let mut second_wire_pos = vec![(0, 0)];

    for i in 0..first_wire.len() {
        let first_pos_diff = compute_distance(&first_wire[i]);
        let second_pos_diff = compute_distance(&second_wire[i]);

        let mut first_current_pos = first_wire_pos[first_wire_pos.len() - 1];
        let mut second_current_pos = second_wire_pos[second_wire_pos.len() - 1];

        let first_target_pos = (first_current_pos.0 + first_pos_diff.0, first_current_pos.1 + first_pos_diff.1);
        let second_target_pos = (second_current_pos.0 + second_pos_diff.0, second_current_pos.1 + second_pos_diff.1);

        while first_current_pos != first_target_pos {
            first_wire_pos.push(first_current_pos);
            if first_pos_diff.0 == 0 {
                first_current_pos.1 += first_pos_diff.1 / first_pos_diff.1.abs();
            } else {
                first_current_pos.0 += first_pos_diff.0 / first_pos_diff.0.abs();
            }
        }

        while second_current_pos != second_target_pos {
            second_wire_pos.push(second_current_pos);
            if first_pos_diff.0 == 0 {
                second_current_pos.1 += second_pos_diff.1 / second_pos_diff.1.abs();
            } else {
                second_current_pos.0 += second_pos_diff.0 / second_pos_diff.0.abs();
            }
        }
    }

//    let intersections = first_wire_pos.intersect(second_wire_pos); 
    
    for first_pos in first_wire_pos.iter() {
        for second_pos in second_wire_pos.iter() {
            if first_pos == second_pos {
                println!("pos: {:?}", first_pos);
                intersections.push(*first_pos);
            }
        }
    }

    let mut min_distance = 100_000_000;
    for intersection in intersections.iter() {
        let distance = manhattan_distance((0, 0), *intersection);
        if distance == 0 {
            continue;
        }
        println!("distance: {}", distance);
        if distance < min_distance {
            min_distance = distance
        }
    }

    min_distance
}
