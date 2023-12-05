use std::fs;

fn main() {
    println!("Hello, world!");
    
    let cal = fs::read_to_string("day01-input.txt")
                                .expect("Tried to read day01-input.txt, failed.");

    let lines = cal.lines().map(String::from).collect();

    println!("{cal}");
}