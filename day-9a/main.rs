use std::fs;
use itertools::Itertools;

fn main() {
    let contents: Vec<i64> = fs::read_to_string("puzzle-input/day-9.txt")
        .unwrap()
        .split('\n')
        .map(|x| x.parse::<i64>())
        .take_while(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();

    for i in 0..contents.len()-25 {
        let nums = &contents[i..i+25];
        let next = contents[i+25];
        let mut combos = nums.iter().combinations(2);

        if !combos.any(|c| c[0] + c[1] == next) {
            println!("Answer: {next}");
            break
        }
    }
}
