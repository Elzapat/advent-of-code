fn main() {
    let input = aoc_base::read_input();
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for line in input.lines() {
        let mut current_values = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut lists = vec![current_values.clone()];

        while !current_values.iter().all(|&v| v == 0) {
            let mut temp_values = Vec::new();

            for (first, second) in current_values.iter().zip(current_values.iter().skip(1)) {
                temp_values.push(second - first);
            }

            lists.push(temp_values.clone());
            current_values = temp_values;
        }

        for i in (1..=lists.len() - 1).rev() {
            let new_number = lists[i - 1][lists[i - 1].len() - 1] + lists[i][lists[i].len() - 1];
            lists[i - 1].push(new_number);
        }

        result_p1 += lists[0][lists[0].len() - 1];

        for i in (1..=lists.len() - 1).rev() {
            let new_number = lists[i - 1][0] - lists[i][0];
            lists[i - 1].insert(0, new_number);
        }

        result_p2 += lists[0][0];
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}")
}
