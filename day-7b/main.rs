use std::{collections::HashMap, fs};
use regex::Regex;

struct Bag {
    id: String,
    capacity: HashMap<String, i32>,
}

fn main() {
    let contents = fs::read_to_string("puzzle-input/day-7.txt").unwrap();

    let bags = contents.lines().map(Bag::parse).collect::<Vec<_>>();

    let count = total_bags("shiny gold", &bags);

    println!("answer: {count}");
}

fn total_bags(pattern: &str, all_bags: &[Bag]) -> i32 {
    let bag = all_bags.iter().find(|x| x.id == pattern).unwrap();
    let mut count = 0;

    for cb in &bag.capacity {
        count += cb.1;
        count += cb.1 * total_bags(cb.0, all_bags);
    }

    count
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
