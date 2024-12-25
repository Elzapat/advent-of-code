fn main() {
    let inp = aoc_base::read_input();
    let input = inp.lines().collect::<Vec<&str>>();
    let mut keys = vec![];
    let mut locks = vec![];

    let mut i = 0;

    while i < input.len() {
        let is_lock = input[i].chars().all(|c| c == '#');
        let mut obj = [0, 0, 0, 0, 0];

        while i < input.len() && !input[i].is_empty() {
            for (ci, c) in input[i].chars().enumerate() {
                if c == '#' {
                    obj[ci] += 1;
                }
            }
            i += 1;
        }

        if is_lock {
            locks.push(obj);
        } else {
            keys.push(obj);
        }

        i += 1
    }

    let mut result = 0;
    let mut visited = vec![];

    for lock in &locks {
        'keys: for key in &keys {
            if visited.contains(&(lock, key)) {
                continue;
            }
            visited.push((lock, key));

            for i in 0..key.len() {
                if lock[i] + key[i] > 7 {
                    continue 'keys;
                }
            }

            result += 1;
        }
    }

    println!("Part 1: {}", result);
}
