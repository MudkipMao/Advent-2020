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
    let mut seen_values: HashSet<u32> = HashSet::new();
    for s in split {
        let val = s.parse::<u32>().unwrap();
        let diff: u32 = target - val;
        if seen_values.contains(&diff) {
            println!("{} and {} multiplied value is: {}", val, diff, val*diff);
            break;
        }
        seen_values.insert(val);
    }
}

fn part2(filename: String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut numbers: Vec<u32> = contents
        .split("\n")
        .map(|c| c.trim().parse::<u32>().unwrap())
        .collect();

    numbers.sort();
    let mut i = 0;
    while i < numbers.len() {
        if threeSumBody(i, &numbers) {
            break;
        }
        i += 1;
    }
}

fn threeSumBody(i: usize, numbers: &Vec<u32>) -> bool {
    let mut lo = i + 1;
    let mut hi = numbers.len() - 1;
    let i_val = numbers[i];
    let target = 2020;
    while lo < hi {
        let sum = i_val + numbers[lo] + numbers[hi];
        if sum == target {
            let multi = i_val * numbers[lo] * numbers[hi];
            println!("{}, {}, {}, are nums, answer is {}", i_val, numbers[lo], numbers[hi], multi);
            return true;
        }
        if sum > target {
            hi -= 1;
        }
        else {
            lo += 1;
        }
    }

    return false;
}
