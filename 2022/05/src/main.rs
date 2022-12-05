fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    const N: u32 = 9;

    let mut stacks = Vec::new();

    let mut i = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    for _ in 0..N {
        stacks.push(Vec::new());
    }

    while !lines[i].is_empty() {
        let mut idx = 0;
        while let Some(mut pos) = lines[i][idx..].find('[') {
            pos += idx + 1;
            idx = pos;
            pos = match pos {
                1 => 1,
                5 => 2,
                9 => 3,
                13 => 4,
                17 => 5,
                21 => 6,
                25 => 7,
                29 => 8,
                33 => 9,
                _ => unreachable!(),
            };
            stacks[pos - 1].insert(0, lines[i].chars().nth(idx).unwrap());
        }

        i += 1;
    }

    i += 1;

    let re = regex::Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    while i < lines.len() {
        /*
        if let Some(captures) = re.captures(lines[i]) {
            for _ in 0..captures[1].parse::<u32>().unwrap() {
                let boite = stacks[captures[2].parse::<usize>().unwrap() - 1]
                    .pop()
                    .unwrap();
                stacks[captures[3].parse::<usize>().unwrap() - 1].push(boite);
            }
        }
        */
        if let Some(captures) = re.captures(lines[i]) {
            let n = captures[1].parse::<usize>().unwrap();
            let boite_len = stacks[captures[2].parse::<usize>().unwrap() - 1].len();
            let mut to_add = Vec::new();
            for boite in
                stacks[captures[2].parse::<usize>().unwrap() - 1].drain(boite_len - n..boite_len)
            {
                to_add.push(boite);
            }

            for boite in to_add {
                stacks[captures[3].parse::<usize>().unwrap() - 1].push(boite);
            }
        }
        i += 1;
    }

    for stack in &stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}
