use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./sample.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
}