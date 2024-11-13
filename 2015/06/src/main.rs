fn main() {
    let input = aoc_base::read_input();
    let mut lights = vec![vec![0; 1000]; 1000];

    for instr in input.lines() {
        let mut parts = instr.split_whitespace().collect::<Vec<&str>>();

        if parts[0] == "turn" {
            parts.remove(0);
        }

        let range_start = parts[1].split(",").map(|r| r.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let range_end = parts[3].split(",").map(|r| r.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        for i in range_start[0]..=range_end[0] {
            for j in range_start[1]..=range_end[1] {
                match parts[0] {
                    "on" => lights[i][j] += 1,
                    "off" => lights[i][j] -= if lights[i][j] == 0 { 0 } else { 1 },
                    "toggle" => lights[i][j] += 2,
                    _ => unreachable!()
                }
            } 
        }
    }

    println!("Part 2: {}", lights.iter().map(|row| row.iter().sum::<i32>()).sum::<i32>());
}
