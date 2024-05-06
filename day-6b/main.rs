use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("puzzle-input/day-6.txt").unwrap();

    let answer: i32 = contents
        .split("\n\n")
        .map(num_yes_answers) // remove all whitespace & collect unique chars
        .sum();

    println!("Answer: {answer}");
}

fn num_yes_answers(s: &str) -> i32 {
    let responses = s.lines();
    let first_response: &str = responses.clone().next().unwrap();

    first_response
        .chars()
        .filter(|ch| responses.clone().all(|line| line.contains(*ch)))
        .count() as i32

}
