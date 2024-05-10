use std::fs;

fn main() {
    let mut contents: Vec<i32> = fs::read_to_string("puzzle-input/day-10.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    //account for device's built-in adapter 
    contents.push(0);
    //account for device's built-in adapter 
    contents.push(*contents.iter().max().unwrap() + 3);

    contents.sort();
    dbg!(&contents);

    let answer = contents
        .windows(2)
        .map(|x| x[1] - x[0])
        .fold((0, 0), |acc, x| {
              if x == 1 {
                  return (acc.0 + 1, acc.1);
              };
              
              if x == 3 {
                  return (acc.0, acc.1 + 1);
              };

              (acc.0, acc.1)
            }
        );

    dbg!(answer);
}
