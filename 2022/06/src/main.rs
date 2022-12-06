fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    const N_UNIQUE_CHARS: usize = 14;

    for line in &lines {
        for i in 0..line.len() - N_UNIQUE_CHARS {
            if line[i..i + N_UNIQUE_CHARS]
                .chars()
                .collect::<std::collections::HashSet<char>>()
                .len()
                == N_UNIQUE_CHARS
            {
                println!("{}", i + N_UNIQUE_CHARS);
                return;
            }
        }
    }
}
