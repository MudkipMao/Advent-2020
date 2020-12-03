use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];
    let filename = &args[2];
    let num_down = &args[3];
    let num_right = &args[4];
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
    let mut col_index = 0;
    let mut tree_count = 0;
    let line_len = split[0].len();

    for line in &split {
        if line.chars().nth(col_index).unwrap() == '#'{
            tree_count += 1;
        }
        col_index = (col_index + 3) % line_len;
    }
    println!("Number of trees is {}, Line len is {}", tree_count, line_len);
}

fn part2(filename: String, num_down: u32, num_right: u32) {
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