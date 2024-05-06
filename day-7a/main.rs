use std::{collections::HashMap, fs};
use regex::Regex;

struct Bag {
    id: String,
    capacity: HashMap<String, i32>,
}

fn main() {
    let contents = fs::read_to_string("puzzle-input/day-7.txt").unwrap();

    let bags = contents.lines().map(Bag::parse).collect::<Vec<_>>();

    let mut count = 0;
    for bag in &bags {
        if bags_contains("shiny gold", bag, &bags) {
            count += 1;
        }
    }

    println!("answer: {count}");
}

fn bags_contains(pattern: &str, bag: &Bag, all_bags: &[Bag]) -> bool {
    for cb in &bag.capacity {
        if cb.0 == pattern {
            return true;
        }

        if bags_contains(pattern, all_bags.iter().find(|x| x.id == *cb.0).unwrap(), all_bags) {
            return true;
        }
    }

    false
}

impl Bag {
    fn parse(s: &str) -> Self {
        let reg_id = Regex::new(r#"^(.*) bags contain (.*)$"#).unwrap();
        let reg_cb = Regex::new(r#"(\d) (.*) bag"#).unwrap();
        let captures = reg_id.captures(s).unwrap();

        let id = captures.get(1).unwrap().as_str().to_owned();
        let mut capacity = HashMap::new();

        for cb in captures.get(2).unwrap().as_str().split(", ") {
            if cb == "no other bags." {
                break
            }

            let cb_captures = reg_cb.captures(cb).unwrap();
            let cb_id: String = cb_captures.get(2).unwrap().as_str().to_owned();
            let num: i32 = cb_captures.get(1).unwrap().as_str().parse().unwrap();

            capacity.insert(cb_id, num);
        }

        Self {
            id,
            capacity,
        }
    }
}
