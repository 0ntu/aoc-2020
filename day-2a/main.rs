use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let mut res = 0;

    for line in read_to_string("./puzzle-input/day-2.txt").unwrap().lines() {
        if valid_password(&line) {
            res += 1;
        }
    }

    println!("Answer: {}", res);
}

fn valid_password(s: &str) -> bool {
    let reg: Regex = Regex::new(r#"^(\d+)-(\d+) (.): (.+)$"#).unwrap();
    let (_, [lower, upper, ch, pass]) = reg.captures(s).unwrap().extract();

    let count = pass
        .chars()
        .filter(|c| *c == ch.chars().next().unwrap())
        .count();

    if count >= lower.parse().unwrap() && count <= upper.parse().unwrap() {
        println!(
            "Lower: {}, Upper: {}, Match: {}, Password: {}",
            lower, upper, ch, pass
        );
        true
    } else {
        false
    }
}
