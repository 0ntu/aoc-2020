use std::fs::read_to_string;

fn main() {
    let contents: Vec<String> = read_to_string("puzzle-input/day-3.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut num_trees = 0;
    let mut row = 0;
    let mut col = 0;

    const OFFSET_RIGHT: usize = 3;
    const OFFSET_DOWN: usize = 1;

    loop {
        col = (col + OFFSET_RIGHT) % contents[0].chars().count();
        row += OFFSET_DOWN;

        if row >= contents.len() {
            break;
        }

        if contents[row].chars().nth(col).unwrap() == '#' {
            num_trees += 1;
        }
    }

    println!("Answer: {}", num_trees);
}
