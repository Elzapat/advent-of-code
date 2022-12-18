#[derive(Debug, Copy, Clone)]
struct Cube {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

const DIRS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
];

fn sides_free((x, y, z): (i32, i32, i32), cubes: &[(i32, i32, i32)]) -> i32 {
    let mut sides_free = 6;
    for (dx, dy, dz) in &DIRS {
        if cubes.contains(&(x + dx, y + dy, z + dz)) {
            sides_free -= 1;
        }
    }
    sides_free
}

fn is_air_pocket(
    cube: (i32, i32, i32),
    cubes: &[(i32, i32, i32)],
    area: &Cube,
    visited: &mut Vec<(i32, i32, i32)>,
) -> bool {
    visited.push(cube);

    if cube.0 == area.min_x
        || cube.0 == area.max_x
        || cube.1 == area.min_y
        || cube.1 == area.max_y
        || cube.2 == area.min_z
        || cube.2 == area.max_z
    {
        false
    } else {
        let mut trapped = true;
        for (dx, dy, dz) in &DIRS {
            let neighbour = (cube.0 + dx, cube.1 + dy, cube.2 + dz);
            if visited.contains(&neighbour) || cubes.contains(&neighbour) {
                trapped &= true;
            } else if !cubes.contains(&neighbour) {
                trapped &= is_air_pocket(neighbour, cubes, area, visited);
            } else {
                trapped &= false;
            }
        }
        trapped
    }
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let cubes = input
        .lines()
        .map(|l| {
            let mut s = l.split(",");
            (
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32, i32)>>();

    let mut result = 0;

    for &cube in &cubes {
        result += sides_free(cube, &cubes);
    }

    println!("Part 1: {result}");

    let max_x = cubes.iter().map(|c| c.0).max().unwrap();
    let min_x = cubes.iter().map(|c| c.0).min().unwrap();
    let max_y = cubes.iter().map(|c| c.1).max().unwrap();
    let min_y = cubes.iter().map(|c| c.1).min().unwrap();
    let max_z = cubes.iter().map(|c| c.2).max().unwrap();
    let min_z = cubes.iter().map(|c| c.2).min().unwrap();

    let area = Cube {
        max_x,
        min_x,
        max_y,
        min_y,
        max_z,
        min_z,
    };

    let mut trapped_air = vec![];

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                let cube = (x, y, z);

                let mut visited = vec![];
                if !trapped_air.contains(&cube) && is_air_pocket(cube, &cubes, &area, &mut visited)
                {
                    trapped_air.append(&mut visited);
                }
            }
        }
    }

    let mut result = 0;

    for (x, y, z) in &cubes {
        let mut sides_free = 6;

        for (dx, dy, dz) in &DIRS {
            let neighbour = (x + dx, y + dy, z + dz);
            if cubes.contains(&neighbour) || trapped_air.contains(&neighbour) {
                sides_free -= 1;
            }
        }

        result += sides_free;
    }

    println!("Part 2: {result}");
}
