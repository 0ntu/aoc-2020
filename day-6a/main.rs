use std::collections::HashMap;
use std::fs;

struct Bag {
    id: String,
    capacity: HashMap<String, i32>,
}

fn main() {
    let contents = fs::read_to_string("puzzle-input/day-7.txt").unwrap();
    let bags = contents.lines().map(Bag::parse).collect::<Vec<_>>();
}

impl Bag {
    fn parse(s: &str) -> Self {
        let mut it = s.trim().split(" bags contain ");

        let id = it.next().unwrap().into();
        let capacity = HashMap::new();

        it.next().unwrap().split(',');

        Self {
            id: id,
            capacity: capacity,
        }
    }

    fn contains(&self, id: &str) -> bool {
        if self.id == id {
            return true;
        }       

        for contained_bag in &self.capacity {
            if contained_bag.0.contains(id) {
                return true;
            }
        }

        false
    }
}
