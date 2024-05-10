use itertools::Itertools;
use std::fs;

fn main() {
    let contents: Vec<i64> = fs::read_to_string("puzzle-input/day-9.txt")
        .unwrap()
        .split('\n')
        .map(|x| x.parse::<i64>())
        .take_while(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();

    let mut invalid_number = None;

    for i in 0..contents.len() - 25 {
        let nums = &contents[i..i + 25];
        let next = contents[i + 25];
        let mut combos = nums.iter().combinations(2);

        if !combos.any(|c| c[0] + c[1] == next) {
            invalid_number = Some(next);
            break;
        }
    }

    let invalid_number = invalid_number.unwrap();

    let mut bottom = 0;
    let mut top = 0;
    let answer = loop {
        debug_assert!(bottom <= top);
        let res: i64 = contents[bottom..=top].iter().sum();

        match res.cmp(&invalid_number) {
            std::cmp::Ordering::Equal => {
                break (contents[bottom..=top].iter().max().unwrap()
                    + contents[bottom..=top].iter().min().unwrap())
            }
            std::cmp::Ordering::Less => top += 1,
            std::cmp::Ordering::Greater => bottom += 1,
        }
    };

    println!("Answer: {answer:?}")
}
