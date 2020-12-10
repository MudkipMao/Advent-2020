use std::env;
use std::fs;
use std::cmp;

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
    let file_iter = contents.split("\n");
    let mut max_seat_id = 0;
    for line in file_iter {
        let row_slice = &line[0..7];
        let col_slice = &line[7..10];

        let mut row = 0;
        for c in row_slice.chars() {
            row <<= 1;
            if c == 'B' {
                row |= 1;
            }
        }

        let mut col = 0;
        for c in col_slice.chars() {
            col <<= 1;
            if c == 'R' {
                col |= 1;
            }
        }
        let seat_id = (row * 8) + col;
        max_seat_id = cmp::max(max_seat_id, seat_id);
    }
    println!("Max seat ID is {}", max_seat_id);
}

fn part2(filename: String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let file_iter = contents.split("\n");
    let mut seat_map = vec![vec![false; 8]; 128];
    for line in file_iter {
        let row_slice = &line[0..7];
        let col_slice = &line[7..10];

        let mut row = 0;
        for c in row_slice.chars() {
            row <<= 1;
            if c == 'B' {
                row |= 1;
            }
        }

        let mut col = 0;
        for c in col_slice.chars() {
            col <<= 1;
            if c == 'R' {
                col |= 1;
            }
        }

        seat_map[row][col] = true;
    }
    // Haha solved this part by visually printing out the chart and seeing the empty seat in console
    for (i, r) in seat_map.iter().enumerate() {
        print!("{}: ", i);
        for c in r {
            print!("{} ", c);
        }
        println!();
    }
}