use aoc_base::Point;

#[derive(Debug, Clone)]
struct Region {
    w: usize,
    l: usize,
    presents: Vec<usize>,
}

fn main() {
    let input = aoc_base::read_input();
    let lines = input.lines();
    let mut shapes = vec![];
    let mut regions = vec![];

    for i in 0..=5 {
        let mut lines = lines.clone().skip(i * 5).take(5).skip(1);
        let mut shape = vec![];

        for row in 0..3 {
            let line = lines.next().unwrap();

            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    shape.push(Point { r: row, c: col });
                }
            }
        }

        shapes.push(shape);
    }

    for line in lines.skip(30) {
        let mut parts = line.split(": ");
        let size = parts.next().unwrap();
        let presents = parts.next().unwrap();

        regions.push(Region {
            w: size[0..2].parse::<usize>().unwrap(),
            l: size[3..5].parse::<usize>().unwrap(),
            presents: presents
                .split_whitespace()
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        });
    }

    let mut result_p1 = 0;

    for region in &regions {
        let size = region.w * region.l;

        let total_presents_size = region
            .presents
            .iter()
            .enumerate()
            .map(|(i, n)| shapes[i].len() * n)
            .sum::<usize>();

        if size >= total_presents_size {
            result_p1 += 1;
        }
    }

    println!("Part 1: {result_p1}");
}
