use std::fs;
use itertools::Itertools;

fn main() {
    let mut contents: Vec<i32> = fs::read_to_string("puzzle-input/day-10.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    contents.sort();

    let combs = contents.combinations_with_replacement()

}

fn valid_sequence() -> bool {
    true
}
