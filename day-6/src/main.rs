use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

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
    let file_iter: Vec<&str> = contents.split("\n\n").collect();
    let mut total_count = 0;
    // Iterate over groups per file
    for group in file_iter {
        let person_iter: Vec<&str> = group.split("\n").collect();
        let mut seen_values: HashMap<char, usize> = HashMap::new();
        let group_size = person_iter.len();
        // Iterate over person per group
        for person in person_iter {
            // Iterate over answers per person
            for c in person.chars() {
                // print!("person is {}, groupsize is {}, curr char is {}", person, group_size, c);
                match seen_values.get(&c) {
                    Some(answer) => {
                        let new_val = answer + 1;
                        seen_values.insert(c, new_val);
                        if new_val == group_size {
                            total_count += 1;
                        }
                    }
                    None => {
                        seen_values.insert(c, 1);
                        if group_size == 1 {
                            total_count += 1;
                        }
                    }
                }
                // println!(" count is {}", seen_values.get(&c).unwrap());
            }
        }
    }
    println!("Total count is {}", total_count);
}