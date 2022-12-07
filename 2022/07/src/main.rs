// 1291969 52067 1121376
use std::collections::HashMap;

fn main() {
    // let input = "";
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    let mut current_directory = "root/".to_owned();
    let mut directories = HashMap::new();
    let mut i = 1;

    'main: loop {
        let line = &lines[i];
        if line.starts_with('$') {
            match &line[2..4] {
                "cd" => match &line[5..] {
                    ".." => {
                        let pos = current_directory[..current_directory.len() - 1]
                            .rfind('/')
                            .unwrap();
                        current_directory.truncate(pos);
                        current_directory.push('/');
                    }
                    filename => {
                        current_directory = format!("{current_directory}{filename}/");
                    }
                },
                "ls" => {
                    i += 1;
                    let mut line = lines[i].clone();

                    while !line.starts_with('$') {
                        if !line.starts_with("dir") {
                            let mut split = line.split(' ');
                            let size = split.next().unwrap().parse::<u32>().unwrap();
                            let name = split.next().unwrap().to_owned();

                            let mut temp_dir = current_directory.clone();
                            while let Some(pos) = temp_dir[..temp_dir.len() - 1].rfind('/') {
                                *directories.entry(temp_dir.clone()).or_insert(0) += size;

                                temp_dir.truncate(pos + 1);
                            }
                            *directories.entry("root/".to_owned()).or_insert(0) += size;
                            // for directory in current_directory.split('/').map(|d| d.to_owned()) {
                            //     if directory.is_empty() {
                            //         continue;
                            //     }
                            //
                            //     *directories.entry(directory.clone()).or_insert(0) += size;
                            // }
                        }

                        i += 1;
                        if i >= lines.len() {
                            break 'main;
                        }
                        line = lines[i].clone();
                    }

                    i -= 1;
                }
                _ => unreachable!(),
            }
        }

        i += 1;
        if i >= lines.len() {
            break;
        }
    }

    let mut result = 0;

    for (directory, size) in &directories {
        // println!("{directory} {size}");
        if *size <= 100000 {
            result += size;
        }
    }
    println!("Part 1: {result}");

    let free_space = 70000000 - directories["root/"];
    let mut smallest = 1000000000;

    for (_, &size) in &directories {
        if size + free_space >= 30000000 && size < smallest {
            smallest = size;
        }
    }
    println!("Part 2: {smallest}");
}
