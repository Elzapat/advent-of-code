use aoc_base::Point;
use pathfinding::directed::dijkstra::dijkstra;

fn main() {
    let bytes = aoc_base::read_input()
        .lines()
        .map(|l| {
            let mut parts = l.split(",");
            Point::new(
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<Point>>();

    let bytes1024 = &bytes[0..1024];

    let size = 70 + 1;
    let start = Point::new(0, 0);
    let end = Point::new(70, 70);

    let path = dijkstra(
        &start,
        |pos: &Point| {
            pos.get_4_neighbours(size, size)
                .iter()
                .filter(|n| !bytes1024.contains(&Point::new(n.c, n.r)))
                .map(|n| (*n, 1))
                .collect::<Vec<_>>()
        },
        |pos| *pos == end,
    );

    println!("Part 1: {}", path.unwrap().0.len() - 1);

    for bytes_len in 1024..bytes.len() {
        let bytes = &bytes[0..bytes_len];

        let path = dijkstra(
            &start,
            |pos: &Point| {
                pos.get_4_neighbours(size, size)
                    .iter()
                    .filter(|n| !bytes.contains(&Point::new(n.c, n.r)))
                    .map(|n| (*n, 1))
                    .collect::<Vec<_>>()
            },
            |pos| *pos == end,
        );

        if path.is_none() {
            println!(
                "Part 2: {},{}",
                bytes[bytes_len - 1].r,
                bytes[bytes_len - 1].c
            );
            break;
        }
    }
}
