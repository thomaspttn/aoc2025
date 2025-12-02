fn number_is_valid(num: &str) -> bool {
    // for part 1 here, must repeat exactly twice
    if num.len() % 2 != 0 {
        return true;
    }
    let mut num_is_valid = true;

    // worst case tessalation size is half the length of the number
    let max_tessalation_size = num.len() / 2;

    // speaking english: a number is invalid if ANY tessalation exists for it

    // for each possible window length
    for window_length in max_tessalation_size..max_tessalation_size + 1 {
        // skip anything which does not evenly divide
        if num.len() % window_length != 0 {
            continue;
        }
        let mut keep_tessalating = true;

        let substr = &num[0..window_length];
        for start_index in (0..num.len()).step_by(window_length) {
            let current_substr = &num[start_index..start_index + window_length];

            // if we find a mismatch, stop tessalating
            if current_substr != substr {
                keep_tessalating = false;
                break;
            }
        }
        // if we finished the loop and were still tessalating, the number is invalid
        if keep_tessalating {
            num_is_valid = false;
            break;
        }
    }
    num_is_valid
}

fn main() {
    // read in input.txt as a string
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");

    // each comma is a range, split by comma
    let ranges: Vec<&str> = input.trim().split(',').collect();

    let mut sum_of_invalid_numbers = 0u64;

    // each range is split by a hyphen, split each range into a tuple of (start, end)
    for range in ranges {
        let bounds: Vec<&str> = range.split('-').collect();
        let start = bounds[0].parse::<u64>().unwrap();
        let end = bounds[1].parse::<u64>().unwrap();
        println!("Range: {} - {}", start, end);

        // for each number in range, check if it meets criteria
        for num in start..=end {
            println!("Current number: {}", num);
            if !number_is_valid(&num.to_string()) {
                println!("Invalid number found: {}", num);
                sum_of_invalid_numbers += num as u64;
            }
        }
    }

    println!("Sum of invalid numbers: {}", sum_of_invalid_numbers);
}
