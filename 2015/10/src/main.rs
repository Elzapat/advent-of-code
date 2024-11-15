fn main() {
    let mut input = aoc_base::read_input().trim().to_string();

    for j in 0..50 {
        let mut digit_counter = 0;
        let mut output = "".to_string();
        let mut i = 0;
    
        while i < input.len() {
            let current_digit = input.chars().nth(i).unwrap();
    
            while i < input.len() && current_digit == input.chars().nth(i).unwrap() {
                i += 1;
                digit_counter += 1;
            }
    
            output += &digit_counter.to_string();
            output.push(current_digit);
               
            digit_counter = 0;
        }
    
        input = output;
    }

    println!("Part 1: {}", input.len());
}
