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
    let file_iter = contents.split("\n\n");
    let mut total_count = 0;
    // Iterate over groups per file
    for group in file_iter {
        let person_iter = group.split("\n");
        let mut seen_values: HashSet<char> = HashSet::new();
        // Iterate over person per group
        for person in person_iter {
            // Iterate over answers per person
            for c in person.chars() {
                seen_values.insert(c);
            }
        }
        total_count += seen_values.len();
    }
    println!("Total count is {}", total_count);
}

fn part2(filename: String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let file_iter = contents.split("\n\n");
    let mut total_count = 0;
    // Iterate over groups per file
    for group in file_iter {
        let person_iter = group.split("\n");
        let mut seen_values: HashSet<char> = HashSet::new();
        // Iterate over person per group
        for person in person_iter {
            // Iterate over answers per person
            for c in person.chars() {
                seen_values.insert(c);
            }
        }
        total_count += seen_values.len();
    }
    println!("Total count is {}", total_count);
}