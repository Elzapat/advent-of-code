fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let program = file.split(",").map(|x| x.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    {
        let mut computer = intcode_computer::IntcodeComputer::new(program.clone(), vec![1]);
        computer.run();

        println!("{:?}", computer);
        println!("Part 1: {}", computer.outputs[0]);
    }

    {
        println!("Part 2: {}", 0);
    }
}
