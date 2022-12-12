#[derive(Copy, Clone, Debug)]
struct Node {
    height: u32,
    parent_x: Option<u32>,
    parent_y: Option<u32>,
    distance: u32,
    passed: bool,
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    let mut grid = vec![];

    for line in &lines {
        let mut row = vec![];

        for s in line.chars() {
            let height = match s {
                'S' => 'a' as u32,
                'E' => 'z' as u32 + 1,
                height => height as u32,
            };

            row.push(Node {
                height,
                parent_x: None,
                parent_y: None,
                distance: if s == 'S' { 0 } else { std::u32::MAX },
                passed: false,
            })
        }

        grid.push(row);
    }

    println!("Part 1: {}", solve(grid.clone()).unwrap());

    for row in &mut grid {
        for node in row {
            if node.height == 'a' as u32 {
                node.distance = 0;
            }
        }
    }

    println!("Part 2: {}", solve(grid).unwrap());
}

fn find_smallest_dist(grid: &[Vec<Node>]) -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;
    let mut min_distance = std::u32::MAX;

    for (i, row) in grid.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if !node.passed && node.distance < min_distance {
                min_distance = node.distance;
                x = j;
                y = i;
            }
        }
    }

    (x, y)
}

fn solve(mut grid: Vec<Vec<Node>>) -> Option<u32> {
    while !grid.iter().all(|row| row.iter().all(|node| node.passed)) {
        let (cur_x, cur_y) = find_smallest_dist(&grid);
        grid[cur_y][cur_x].passed = true;
        let cur_node = grid[cur_y][cur_x];

        if cur_node.height == 'z' as u32 + 1 {
            let mut trav_node = cur_node;
            let mut result = 0;

            while trav_node.parent_x.is_some() {
                result += 1;
                trav_node = grid[trav_node.parent_y.unwrap() as usize]
                    [trav_node.parent_x.unwrap() as usize];
            }

            return Some(result);
        }

        let mut neighbours = vec![];

        // Up
        if cur_y >= 1
            && !grid[cur_y - 1][cur_x].passed
            && cur_node.height >= grid[cur_y - 1][cur_x].height - 1
        {
            neighbours.push((cur_y - 1, cur_x));
        }

        // Down
        if cur_y < grid.len() - 1
            && !grid[cur_y + 1][cur_x].passed
            && cur_node.height >= grid[cur_y + 1][cur_x].height - 1
        {
            neighbours.push((cur_y + 1, cur_x));
        }

        // Left
        if cur_x >= 1
            && !grid[cur_y][cur_x - 1].passed
            && cur_node.height >= grid[cur_y][cur_x - 1].height - 1
        {
            neighbours.push((cur_y, cur_x - 1));
        }

        // Right
        if cur_x < grid[0].len() - 1
            && !grid[cur_y][cur_x + 1].passed
            && cur_node.height >= grid[cur_y][cur_x + 1].height - 1
        {
            neighbours.push((cur_y, cur_x + 1));
        }

        for (ny, nx) in &neighbours {
            let mut neighbour = &mut grid[*ny][*nx];
            let new_dist = cur_node.distance + 1;

            if new_dist < neighbour.distance {
                neighbour.distance = new_dist;
                neighbour.parent_y = Some(cur_y as u32);
                neighbour.parent_x = Some(cur_x as u32);
            }
        }
    }

    None
}
