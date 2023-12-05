use std::io::{BufRead, BufReader};
use std::fs::File;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {

    let f = File::open("../day02/input.txt").expect("error opening file");
    let r = BufReader::new(f);
    println!("Number of winning games: {}", parse_games(r));
}

fn parse_games(input: BufReader<File>) -> i32{

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    let mut winning: i32 = 0;

    for (idx, line) in input.lines().enumerate() {
        let mut failed: bool = false;
        let line: String = line.expect("error reading line");
        let parts: Vec<&str> = line.split(':').collect();
        let grabs: Vec<&str> = parts[1].trim().split(';').collect();
        
        r = 0;
        g = 0;
        b = 0;

        for grab in grabs.iter().enumerate() {
            for color in grab.1.trim().split(", ").enumerate() {
                let tc = color.1.trim().split(' ').collect::<Vec<&str>>();
                let q = tc[0].parse::<i32>().unwrap();
                let c: &str = tc[1];
                if( c == "red" && q > MAX_RED) || (c == "green" && q > MAX_GREEN) || (c == "blue" && q > MAX_BLUE) {
                    failed = true;
                    break;
                }
            }
        }
        if failed == false{
            winning += idx as i32 + 1;
        }
        println!("Game {} passed", idx + 1);
    }

    winning
}