// meta goal here right: now instead of ones and tens we have all the way up to 10^12
// start by finding highest digit between [0..n-12], mark index as i
// then find highest digit between [i+1..n-11], mark index as j
// then find highest digit between [j+1..n-10], mark index as k
// ...

fn modified_find_highest_digit(line: &Vec<u32>, start_idx: u32, end_idx: u32) -> (u32, u32) {
    // goal here is to find the highest digit in line between start_idx and end_idx
    // returns the highest digit and the index it was found at
    let mut highest_digit = 0;
    let mut highest_digit_idx = start_idx;

    for idx in start_idx..end_idx {
        let digit = line[idx as usize];
        if digit > highest_digit {
            highest_digit = digit;
            highest_digit_idx = idx;
        }
    }
    (highest_digit, highest_digit_idx)
}

fn get_highest_joltage(line: &str) -> u32 {
    // split on char, parse each as u32, collect into a vector
    let line_numbers = line
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut highest_joltage = 0;
    let mut start_idx = 0u32;
    for power in 12..=0 {
        let range_end = line_numbers.len() as u32 - power;
        let (highest_digit, highest_digit_idx) =
            modified_find_highest_digit(&line_numbers, start_idx, range_end);

        start_idx = highest_digit_idx + 1;
        highest_joltage += highest_digit * 10u32.pow(power);
    }
    highest_joltage
}

fn main() {
    // read each line in a file called input.txt
    let input = std::fs::read_to_string("input.txt").expect("Failed to read file");

    let mut total_joltage = 0;

    // for line in input, print the line
    for line in input.lines() {
        println!("{}", line);

        total_joltage += get_highest_joltage(line);
    }

    println!("Total joltage: {}", total_joltage);
}
