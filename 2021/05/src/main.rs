#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Line {
    p1: Point,
    p2: Point,
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines = file
        .lines()
        .map(|line| {
            let mut points = line.split("->");
            let mut p1 = points.next().unwrap().trim().split(",");
            let mut p2 = points.next().unwrap().trim().split(",");

            Line {
                p1: Point {
                    x: p1.next().unwrap().parse::<u32>().unwrap(),
                    y: p1.next().unwrap().parse::<u32>().unwrap(),
                },
                p2: Point {
                    x: p2.next().unwrap().parse::<u32>().unwrap(),
                    y: p2.next().unwrap().parse::<u32>().unwrap(),
                },
            }
        })
        .collect::<Vec<Line>>();

    {
        let lines = lines.clone();
        let mut points_n: std::collections::HashMap<Point, u32> = std::collections::HashMap::new();

        for line in &lines {
            if line.p1.x == line.p2.x {
                let (lower, upper) = if line.p1.y > line.p2.y {
                    (line.p2.y, line.p1.y)
                } else {
                    (line.p1.y, line.p2.y)
                };

                for i in lower..=upper {
                    if let None = points_n.get(&Point { x: line.p1.x, y: i }) {
                        points_n.insert(Point { x: line.p1.x, y: i }, 1);
                    } else {
                        *points_n.get_mut(&Point { x: line.p1.x, y: i }).unwrap() += 1;
                    }
                }
            } else if line.p1.y == line.p2.y {
                let (lower, upper) = if line.p1.x > line.p2.x {
                    (line.p2.x, line.p1.x)
                } else {
                    (line.p1.x, line.p2.x)
                };

                for i in lower..=upper  {
                    if let None = points_n.get(&Point { x: i, y: line.p1.y }) {
                        points_n.insert(Point { x: i, y: line.p1.y }, 1);
                    } else {
                        *points_n.get_mut(&Point { x: i, y: line.p1.y }).unwrap() += 1;
                    }
                }
            }
        }

        let mut res = 0;
        for (_, n) in &points_n {
            if *n >= 2 {
                res += 1;
            }
        }

        println!("Part 1: {}", res);
    }

    {
        let lines = lines.clone();
        let mut board = [[0_u32; 1000]; 1000];

        for line in &lines {
            if line.p1.x == line.p2.x {
                let (lower, upper) = if line.p1.y > line.p2.y {
                    (line.p2.y, line.p1.y)
                } else {
                    (line.p1.y, line.p2.y)
                };

                for y in lower..=upper {
                    board[y as usize][line.p1.x as usize] += 1;
                }
            } else if line.p1.y == line.p2.y {
                let (lower, upper) = if line.p1.x > line.p2.x {
                    (line.p2.x, line.p1.x)
                } else {
                    (line.p1.x, line.p2.x)
                };

                for x in lower..=upper  {
                    board[line.p1.y as usize][x as usize] += 1;
                }
            } else {
                let mut x = line.p1.x;
                let mut y = line.p1.y;

                loop {
                    board[y as usize][x as usize] += 1;

                    if line.p1.x > line.p2.x {
                        if x == 0 {
                            break;
                        }
                        x -= 1;
                        if x < line.p2.x {
                            break;
                        }
                    } else {
                        x += 1;
                        if x > line.p2.x {
                            break;
                        }
                    }

                    if line.p1.y > line.p2.y {
                        if y == 0 {
                            break;
                        }
                        y -= 1;
                        if y < line.p2.y {
                            break;
                        }
                    } else {
                        y += 1;
                        if y > line.p2.y {
                            break;
                        }
                    }
                }
            }
        }

        let mut res = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                if board[y][x] >= 2 {
                    res += 1;
                }
            }
        }

        println!("Part 2: {}", res);
    }
}
