fn main() {
    let input = aoc_base::read_input();

    let mut paper = 0;
    let mut ribbon =  0;

    for line in input.lines() {
        let mut dims = line.split('x').map(|d| d.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        dims.sort();

        let mut faces = vec![dims[0] * dims[1], dims[1] * dims[2], dims[0] * dims[2]];
        faces.sort();

        paper += 3 * faces[0] + 2 * faces[1] + 2 * faces[2];
        ribbon += 2 * dims[0] + 2 * dims[1] + dims[0] * dims[1] * dims[2];
    }

    println!("Part 1: {paper}");
    println!("Part 2: {ribbon}");
}
