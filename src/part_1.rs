fn main() {
    // open input.txt
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut times_pointing_at_zero = 0;
    let mut current_position = 50; // provided as a part of the problem

    // for line in input, print line
    for line in input.lines() {
        // split first char and rest of line
        let direction = &line[0..1];
        let amount = (&line[1..]).parse::<i32>().unwrap();
        match direction {
            "L" => {
                current_position = (current_position - amount) % 100;
            }
            "R" => {
                current_position = (current_position + amount) % 100;
            }
            _ => panic!("Invalid direction"),
        }
        // println!("Current position: {}", current_position);
        if current_position == 0 {
            times_pointing_at_zero += 1;
        }
    }

    println!("Times pointing at zero: {}", times_pointing_at_zero);
}
