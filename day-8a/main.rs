use std::fs;

fn main() {
    let contents = fs::read_to_string("puzzle-input/day-8.txt").unwrap();
    let instructions = contents.lines().collect::<Vec<_>>();

    let mut acc = 0;
    let mut i: i32 = 0;
    let mut visited = Vec::new();
    while i < instructions.len().try_into().unwrap() {
        if visited.contains(&i) {
            break
        } else {
            visited.push(i);
        }

        let args = instructions[i as usize].split_ascii_whitespace().collect::<Vec<_>>();
        let cmd = args[0];
        let val: i32 = args[1].parse().unwrap();

        match cmd {
            "nop" => {},
            "acc" => acc += val,
            "jmp" => {i += val; continue},
            _ => panic!("Invalid cmd"),
        }

        i += 1;
    }

    println!("{acc}");
}
