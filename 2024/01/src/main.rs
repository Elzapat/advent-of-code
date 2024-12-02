fn main() {
    let input = aoc_base::read_input();

    let mut sum = 0;
    let mut sump2 = 0;
    let mut l1 = vec![];
    let mut l2 = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let n1 = parts.next().unwrap().parse::<i32>().unwrap();
        let n2 = parts.next().unwrap().parse::<i32>().unwrap();

        l1.push(n1);
        l2.push(n2);
    }

    l1.sort();
    l2.sort();

    for i in 0..l1.len() {
        sum += (l1[i] - l2[i]).abs();

        let mut n_appear = 0;

        for j in 0..l2.len() {
            if l2[j] == l1[i] {
                n_appear += 1;
            }
        }

        sump2 += l1[i] * n_appear;
    }

    println!("Part 1: {sum}");
    println!("Part 2: {sump2}");
}
