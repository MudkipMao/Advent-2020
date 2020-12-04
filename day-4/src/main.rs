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
    let file_str: Vec<&str> = contents.split("\n\n").collect();
    let mut num_valid_passport = 0;
    // Iterate over passports in file
    for passport in &file_str {
        let passport_split: Vec<&str> = passport.split_whitespace().collect();
        // I will assume that there cannot be any incorrect fields for now
        if passport_split.len() < 7 {
            continue;
        }
        else if passport_split.len() == 8 {
            num_valid_passport += 1;
            continue;
        }
        // Iterate over fields in passport
        let mut cid_found: bool = false;
        for field in passport_split {
            let key_value: Vec<&str> = field.split(":").collect();
            if key_value[0] == "cid" {
                cid_found = true;
                break;
            }
        }
        if !cid_found {
            num_valid_passport += 1;
        }

    }
    println!("Number of valid passports is {} out of {}", num_valid_passport, file_str.len());
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