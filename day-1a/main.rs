use std::fs;
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("./puzzle-input/day-1.txt").expect("Unable to read file");

    let mut entries = Vec::new();

    for s in contents.trim().lines() {
        entries.push(s.parse::<i32>().unwrap());
    }

    for c in entries.iter().combinations(2) {
        if c[0] + c[1] == 2020 {
            println!("Entry: {}", c[0]);
            println!("Entry: {}", c[1]);
            println!("Answer: {}", c[0] * c[1]);
        }

    }


}
