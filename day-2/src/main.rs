use std::env;
use std::fs;

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
    let split: Vec<&str> = contents.split("\n").collect();
    let mut num_valid = 0;
    for line in &split {
        let line_vec: Vec<&str> = line.split(|c| c == ' ' || c == '-').collect();
        let lo = line_vec[0].parse::<u32>().unwrap();
        let hi = line_vec[1].parse::<u32>().unwrap();
        let letter = line_vec[2].chars().nth(0).unwrap();
        let pass = line_vec[3];
        let mut count = 0;
        for ch in pass.chars() {
            if ch == letter {
                count += 1;
            }
        }
        if count >= lo && count <= hi {
            num_valid += 1;
        }
    }
    println!("Number of valid passwords is {}, num of passwords is {}", num_valid, split.len());
}

fn part2(filename: String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split: Vec<&str> = contents.split("\n").collect();
    let mut num_valid = 0;
    for line in &split {
        let line_vec: Vec<&str> = line.split(|c| c == ' ' || c == '-').collect();
        let lo = line_vec[0].parse::<usize>().unwrap() - 1; // Gotta subract 1 because it's not 0 indexed
        let hi = line_vec[1].parse::<usize>().unwrap() - 1;
        let letter = line_vec[2].chars().nth(0).unwrap();
        let lo_letter = line_vec[3].chars().nth(lo).unwrap();
        let hi_letter = line_vec[3].chars().nth(hi).unwrap();
        if lo_letter == hi_letter {
            continue;
        }
        if lo_letter == letter || hi_letter == letter {
            num_valid += 1;
        }
    }
    println!("Number of valid passwords is {}, num of passwords is {}", num_valid, split.len());
}