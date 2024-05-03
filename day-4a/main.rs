use std::fs::read_to_string;

fn main() {
    let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let contents = read_to_string("puzzle-input/day-4.txt")
        .unwrap();

    let answer = contents
        .split("\n\n")
        .map(|entry| {
            entry
                .split_ascii_whitespace()
                .map(|field| field.split(':').next().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|passport| {
            req_fields
                .iter()
                .all(|req_field| passport.contains(req_field))
        })
        .count();

    println!("Answer: {}", answer);
}
