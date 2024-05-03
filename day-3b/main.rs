use std::fs::read_to_string;

fn main() {
    let contents: Vec<String> = read_to_string("puzzle-input/day-3.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let answer = trees_encountered(&contents, 1, 1)
        * trees_encountered(&contents, 3, 1)
        * trees_encountered(&contents, 5, 1)
        * trees_encountered(&contents, 7, 1)
        * trees_encountered(&contents, 1, 2);

    println!("Answer: {}", answer);
}

fn trees_encountered(contents: &[String], offset_right: usize, offset_down: usize) -> i64 {
    let mut num_trees = 0;
    let mut row = 0;
    let mut col = 0;

    loop {
        col = (col + offset_right) % contents[0].chars().count();
        row += offset_down;

        if row >= contents.len() {
            break;
        }

        if contents[row].chars().nth(col).unwrap() == '#' {
            num_trees += 1;
        }
    }

    num_trees
}
