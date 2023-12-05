use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {

    let file_name: &str = "../day01/day01-input.txt";
    let file: File = File::open(file_name).expect("file not found");
    let mut sum: i32;

    let mut reader: BufReader<File> = std::io::BufReader::new(file);

    sum = part_one(&mut reader);
    println!("Cal Value (digits only): {sum}");
}

fn part_one(reader: &mut BufReader<File>) -> i32{ 

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