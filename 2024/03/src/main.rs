use regex::Regex;

fn main() {
    let input = aoc_base::read_input();

    let mut result_p1 = 0;
    let mut result_p2 = 0;

    let mut enabled = true;
    for line in input.lines() {
        let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

        for cap in re.captures_iter(line) {
            if &cap[0] == "do()" {
                enabled = true;
            } else if &cap[0] == "don't()" {
                enabled = false;
            } else if enabled {
                result_p1 += cap[2].to_string().parse::<u32>().unwrap()
                    * cap[3].to_string().parse::<u32>().unwrap();
                result_p2 += cap[2].to_string().parse::<u32>().unwrap()
                    * cap[3].to_string().parse::<u32>().unwrap();
            } else {
                result_p1 += cap[2].to_string().parse::<u32>().unwrap()
                    * cap[3].to_string().parse::<u32>().unwrap();
            }
        }
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}
