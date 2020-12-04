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

fn part2(filename: String) {
    let args: Vec<String> = env::args().collect();
    let num_down = &args[3].parse::<usize>().unwrap();
    let num_right = &args[4].parse::<usize>().unwrap();
    let num_down: usize = num_down.clone();
    let num_right: usize = num_right.clone();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split: Vec<&str> = contents.split("\n").collect();
    let mut row_index = 0;
    let mut col_index = 0;
    let mut tree_count = 0;
    let line_len = split[0].len();

    for line in &split {
        if (row_index % num_down) != 0 {
            row_index += 1;
            continue;
        }
        if line.chars().nth(col_index).unwrap() == '#'{
            tree_count += 1;
        }
        col_index = (col_index + num_right) % line_len;
        row_index += 1;
    }
    println!("Number of trees is {}, Line len is {}", tree_count, line_len);
}