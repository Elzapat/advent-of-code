fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    let mut x = 1;
    let mut cycles = 1;
    let mut screen = vec![];

    for line in &lines {
        end_of_cycle(&mut x, cycles, &mut screen);
        match &line[0..4] {
            "noop" => {
                cycles += 1;
            }
            "addx" => {
                cycles += 1;
                end_of_cycle(&mut x, cycles, &mut screen);
                cycles += 1;
                x += line[5..].parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }

    for i in 0..6 {
        for j in 0..39 {
            print!("{}", &screen[40 * i + j]);
        }
        println!();
    }
}

fn end_of_cycle(x: &mut i32, cycles: i32, screen: &mut Vec<char>) {
    if [*x, *x + 1, *x + 2].iter().any(|s| *s == cycles % 40) {
        screen.push('#');
    } else {
        screen.push('.');
    }
}
