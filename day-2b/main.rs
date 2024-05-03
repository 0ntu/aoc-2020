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
    println!(
        "lower: {}, upper: {}, ch: {}, pass: {}",
        lower, upper, ch, pass
    );

    let ch = ch.chars().next().unwrap();

    if pass.chars().nth(lower.parse::<usize>().unwrap() - 1).is_none()
        || pass.chars().nth(upper.parse::<usize>().unwrap() - 1).is_none()
    {
        return false;
    }

    //what the fuck
    if ch == pass.chars().nth(lower.parse::<usize>().unwrap() - 1).unwrap()
        && ch == pass.chars().nth(upper.parse::<usize>().unwrap() - 1).unwrap()
    {
        return false;
    }

    if ch != pass.chars().nth(lower.parse::<usize>().unwrap() - 1).unwrap()
        && ch != pass.chars().nth(upper.parse::<usize>().unwrap() - 1).unwrap()
    {
        return false;
    }

    true
}
