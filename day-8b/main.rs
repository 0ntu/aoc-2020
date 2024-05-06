use std::fs;

enum Cmd {
    Nop,
    Acc,
    Jmp,
}

struct Instruction {
    cmd: Cmd,
    val: i32,
}

impl Instruction {
    fn flip(&mut self) {
        if matches!(self.cmd, Cmd::Nop) {
            self.cmd = Cmd::Jmp;
        } else if matches!(self.cmd, Cmd::Jmp) {
            self.cmd = Cmd::Nop;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("puzzle-input/day-8.txt").unwrap();
    let mut instructions = contents
        .lines()
        .map(|s| {
            let args = s.split_ascii_whitespace().collect::<Vec<_>>();
            let cmd = args[0];
            let val: i32 = args[1].parse().unwrap();

            let cmd = match cmd {
                "nop" => Cmd::Nop,
                "acc" => Cmd::Acc,
                "jmp" => Cmd::Jmp,
                _ => panic!("Invalid cmd"),
            };

            Instruction { cmd, val }
        })
        .collect::<Vec<_>>();

    let mut possible_corrupted = instructions
        .iter()
        .filter(|istr| matches!(istr.cmd, Cmd::Nop) || matches!(istr.cmd, Cmd::Jmp));

    for corrupted in possible_corrupted {
        unsafe {
            corrupted.flip();
            get_acc(&instructions);
        }
    }
}

fn get_acc(instructions: &[Instruction]) -> Option<i32> {
    let mut acc = 0;
    let mut i: i32 = 0;
    let mut visited = Vec::new();
    while i < instructions.len().try_into().unwrap() {
        if visited.contains(&i) {
            return None;
        } else {
            visited.push(i);
        }

        match instructions[i as usize].cmd {
            Cmd::Nop => {}
            Cmd::Acc => acc += instructions[i as usize].val,
            Cmd::Jmp => {
                i += instructions[i as usize].val;
                continue;
            }
        }

        i += 1;
    }

    Some(acc)
}
