use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];
    let filename = &args[2];
    let part = part.parse::<i32>().unwrap();
    match part {
        1 => part1(filename.to_string()),
        2 => part2(filename.to_string()),
        _ => (),
    }
}

fn part1(filename: String) {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let target = 2020;
    let mut seen_values: HashSet<i32> = HashSet::new();
    for s in split {
        let val = s.parse::<i32>().unwrap();
        let diff: i32 = target - val;
        if seen_values.contains(&diff) {
            println!("{} and {} multiplied value is: {}", val, diff, val*diff);
            break;
        }
        seen_values.insert(val);
    }
}

fn part2(filename: String) {

}