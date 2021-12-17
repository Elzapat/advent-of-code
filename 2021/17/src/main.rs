#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let input = file
        .replace("target area: ", "")
        .replace("x=", "")
        .replace("y=", "")
        .split(",")
        .map(|coords| {
            let coords = coords.trim().split("..").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            (coords[0], coords[1])
        })
        .collect::<Vec<(i32, i32)>>();

    let mut target_points = vec![];
    for x in input[0].0..=input[0].1 {
        for y in input[1].0..=input[1].1 {
            target_points.push(Point { x, y });
        }
    }

    {
        let highest_target_x = input[0].1;
        let lowest_target_y = input[1].0;
        let mut highest_y = 0;
        let mut hit_counter = 0;

        for i in -200..400 {
            for j in -200..400 {
                let mut vel_x = i;
                let mut vel_y = j;
                let mut x = 0;
                let mut y = 0;
                let mut max_y = 0;

                loop {
                    x += vel_x;
                    y += vel_y;

                    if y > max_y {
                        max_y = y;
                    }

                    vel_x += if vel_x > 0 { -1 } else if vel_x < 0 { 1 } else { 0 };
                    vel_y -= 1;

                    if x + vel_x > highest_target_x || y + vel_y < lowest_target_y {
                        break;
                    }
                }

                if max_y > highest_y && target_points.contains(&Point { x, y }) {
                    highest_y = max_y;
                }

                if target_points.contains(&Point { x, y }) {
                    hit_counter += 1;
                }
            }
        }

        println!("Part 1: {}", highest_y);
        println!("Part 2: {}", hit_counter);
    }
}

/*
    {
        const G: f32 = 9.81;

        let mut hit_target = 0;
        let mut inc_x = true;
        let mut highest_y = 0.0_f32;
        let mut x_vel = 0.0_f32;
        let mut y_vel = 0.0_f32;
        let mut dest_x = 0.0_f32;

        while hit_target != 2 {
            // let alpha: f32 = 2.0 * (y_vel / (x_vel + (y_vel.powi(2) + x_vel.powi(2)).sqrt())).atan();
            // let y = -(G / (2.0 * (x_vel.powi(2) + y_vel.powi(2)) * alpha.cos().sqrt())) * dest_x.powi(2) + alpha.tan() * dest_x;

            if y > highest_y {
                highest_y = y;
            }



            x_vel += (inc_x as u32) as f32;
            y_vel += (inc_x as u32) as f32;
            inc_x = !inc_x;
        }

        println!("Part 1: {}", highest_y);
    }
*/
