use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {

    let file_name: &str = "../day01/day01-input.txt";
    let file: File = File::open(file_name).expect("file not found");
    println!("Cal Value (digits only): {}", part_one(std::io::BufReader::new(file)));

    let file_name: &str = "../day01/day01-input.txt";
    let file: File = File::open(file_name).expect("file not found");
    println!("Cal Value (all numbers): {}", part_two(std::io::BufReader::new(file)));
}

/// Calculates the solution for part one of the Advent of Code 2023 day 01 puzzle.
/// 
/// # Arguments
/// 
/// * `reader` - A mutable reference to a `BufReader<File>` that reads the input file.
/// 
/// # Returns
/// 
/// The calculated solution as an `i32`.
fn part_one(reader: BufReader<File>) -> i32{ 

    let mut sum: i32 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line: String = line.expect("error reading line");
        let mut first: i32 = -1;
        let mut second: i32 = -1;
        let n: i32;
        for i in line.chars() {
            if i.is_numeric() {
                if first == -1 {
                    first = i.to_digit(10).unwrap() as i32;
                }
                else{
                    second = i.to_digit(10).unwrap() as i32;
                }
            }
        }
        if second == -1 {
            second = first;
        }
        n = first *  10 + second;
        sum += n;
    }
    
    sum
}

fn part_two(reader: BufReader<File>) -> i32{ 

    let mut sum: i32 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line: String = line.expect("error reading line");
        let word_to_num = [
                ("zero", 0),
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ]
            .iter()
            .copied()
            .collect::<HashMap<&str, i32>>();

        let mut first: i32 = -1;
        let mut second: i32 = -1;
        let n: i32;
        for (idx, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if first == -1 {
                    first = c.to_digit(10).unwrap() as i32;
                }
                else{
                    second = c.to_digit(10).unwrap() as i32;
                }
            } else if c.is_alphabetic() {
                for(key, value) in word_to_num.iter() {
                    if line[idx..].starts_with(key) {
                        if first == -1 {
                            first = *value;
                        } else {
                            second = *value;
                        }
                    }
                }
            }
        }
        if second == -1 {
            second = first;
        }
        n = first *  10 + second;
        sum += n;
    }
    
    sum
}