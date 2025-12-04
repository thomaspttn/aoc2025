fn determine_number_adjacent(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for (dr, dc) in directions.iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}
fn main() {
    // read each line in a file called input.txt
    let input = std::fs::read_to_string("input.txt").expect("Failed to read file");

    // parse into a 2D vector of chars
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // let mut keep_going = true;
    // while keep_going {

    // }

    let mut num_with_less_than_four_touching = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '@' {
                continue;
            }
            let num_adjacent = determine_number_adjacent(&grid, i, j);
            if num_adjacent < 4 {
                num_with_less_than_four_touching += 1;
                println!(
                    "Found position with less than four adjacent '@' at ({}, {})",
                    i, j
                );
            }
        }
    }

    println!(
        "Number of positions with less than four adjacent '@': {}",
        num_with_less_than_four_touching
    );
}
