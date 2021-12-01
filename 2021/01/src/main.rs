fn main() {
    let input: Vec<i32> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {}", (0..input.len() - 1).fold(0, |acc, i| acc + (input[i] < input[i + 1]) as u32));
    println!("Part 2: {}", (0..input.len() - 3).fold(0, |acc, i| acc + (input[i] + input[i + 1] + input[i + 2] < input[i + 1] + input[i + 2] + input[i + 3]) as u32));

    /*
    let mut counter = 0;
    for i in 0..input.len() - 1 {
        if input[i] < input[i + 1] {
            counter += 1
        }
    }
    println!("Part 1: {:?}", counter);

    let mut counter = 0;
    for i in 0..input.len() - 3 {
        if input[i] + input[i + 1] + input[i + 2] < input[i + 1] + input[i + 2] + input[i + 3] {
            counter += 1;
        }
    }
    println!("Part 2: {}", counter);
    */
}
