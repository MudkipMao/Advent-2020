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
        // Iterate over fields in passport
        let mut cid_found: bool = false;
        let mut fields_valid: bool = true;
        for field in &passport_split {
            if !fields_valid {
                break;
            }
            let key_value: Vec<&str> = field.split(":").collect();
            match key_value[0] {
               "byr" => {
                   let age = key_value[1].parse::<usize>().unwrap();
                   if !(age >= 1920 && age <= 2002) {
                       fields_valid = false;
                       break;
                   }
               }
               "iyr" => {
                   let year = key_value[1].parse::<usize>().unwrap();
                   if !(year >= 2010 && year <= 2020) {
                       fields_valid = false;
                       break;
                   }
               }
               "eyr" => {
                   let year = key_value[1].parse::<usize>().unwrap();
                   if !(year >= 2020 && year <= 2030) {
                       fields_valid = false;
                       break;
                   }
               }
               "hgt" => {
                   let len_value = key_value[1].len();
                   let units = key_value[1].chars().nth(len_value - 1).unwrap();
                   let mut metric = false;
                   if units == 'm' {
                       metric = true;
                   }
                   else if units != 'n' { // This means that we're neither cm nor in 
                       fields_valid = false;
                       break;
                   }
                   let mut numeric_height: u32 = 0;
                   for c in key_value[1].chars() {
                       if !c.is_numeric() {
                           break;
                       }
                       numeric_height = (10 * numeric_height) + c.to_digit(10).unwrap();
                   }
                   if metric {
                       if !(numeric_height >= 150 && numeric_height  <= 193) {
                           fields_valid = false;
                       } 
                    } else {
                       if !(numeric_height >= 59 && numeric_height  <= 76) {
                           fields_valid = false;
                   }
                }
                   
               }
               "hcl" => {
                   if key_value[1].len() != 7 {
                       fields_valid = false;
                       break;
                   }
                   if key_value[1].chars().nth(0).unwrap() != '#' {
                       fields_valid = false;
                       break;
                   }
                   let mut sliced = "";
                   unsafe {
                       sliced = key_value[1].get_unchecked(1..key_value[1].len())
                   }
                   match i64::from_str_radix(sliced, 16) {
                       Ok(_num) => (),
                       Err(_e) => fields_valid = false,
                   }
               }
               "ecl" => {
                   match key_value[1] {
                       "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                       _ => fields_valid = false,
                   }
                
               }
               "pid" => {
                   if (key_value[1].len()) != 9 || !key_value[1].parse::<u32>().is_ok() {
                       fields_valid = false;
                   }
               }
               "cid" => {
                   cid_found = true;
               }
               _ => {
                   println!("Unknown key {}!", key_value[0]);
               }
            }
        }
        if (passport_split.len() == 8) && fields_valid {
            num_valid_passport += 1;
        } else if (passport_split.len() == 7) && fields_valid && !cid_found {
            num_valid_passport += 1;
        }
    }
    println!("Number of valid passports is {} out of {}", num_valid_passport, file_str.len());
}