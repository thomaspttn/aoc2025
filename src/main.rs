fn main() {
    // open input.txt
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut times_past_zero = 0;
    let mut current_position = 50; // provided as a part of the problem

    // for line in input, print line
    for line in input.lines() {
        // split first char and rest of line
        let direction = &line[0..1];
        let mut amount = (&line[1..]).parse::<i32>().unwrap();
        if direction == "L" {
            amount *= -1;
        }

        // too complicated, just simulate the clicks
        for _ in 0..amount.abs() {
            if amount > 0 {
                current_position += 1;
            } else {
                current_position -= 1;
            }

            if current_position > 99 {
                current_position = 0;
            } else if current_position < 0 {
                current_position = 99;
            }

            if current_position == 0 {
                times_past_zero += 1;
            }
        }
    }

    println!("Times pointing at zero: {}", times_past_zero);
}
