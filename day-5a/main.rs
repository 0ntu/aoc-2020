use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("puzzle-input/day-5.txt").unwrap();

    let answer = contents.lines().map(get_seat_id).max().unwrap();
    println!("Answer: {answer}");

}

fn get_seat_id(s: &str) -> i32 {
    let mut lower_row = 0;
    let mut upper_row = 127;
    let mut lower_column = 0;
    let mut upper_column = 7;

    for c in s.chars() {
        let total_seats = upper_row - lower_row + 1;
        let total_columns = upper_column - lower_column + 1;

        match c {
            'F' => upper_row -= total_seats / 2,
            'B' => lower_row += total_seats / 2,
            'L' => {assert_eq!(lower_row, upper_row); upper_column -= total_columns / 2},
            'R' => {assert_eq!(lower_row, upper_row); lower_column += total_columns / 2},
            _ => panic!("Unexpected character!"),
        }
    }

    assert_eq!(lower_row, upper_row);
    assert_eq!(lower_column, upper_column);
    lower_row * 8 + lower_column
}
