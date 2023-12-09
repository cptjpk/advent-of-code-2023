use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut file = File::open("./input.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut running_score: i32 = 0;

    for (_, line) in lines.iter().enumerate() {
        let card: Vec<&str> = line.split(": ").collect();
        let numbers: Vec<&str> = card[1].split(" | ").collect::<Vec<&str>>();
        let mut wnums: Vec<i32> = Vec::new();
        let mut pnums: Vec<i32> = Vec::new();
        wnums.extend(numbers[0].split_whitespace()
            .filter_map(|num_str| num_str.parse::<i32>().ok()));
        pnums.extend(numbers[1].split_whitespace()
            .filter_map(|num_str| num_str.parse::<i32>().ok()));
        running_score += check_winners(&wnums, &pnums);
    }

    println!("Final Score: {}", running_score);
    
}

fn check_winners(winning_numbers: &Vec<i32>, picked_numbers: &Vec<i32>) -> i32{
    let wn: HashSet<_> = winning_numbers.into_iter().collect();
    let pn: HashSet<_> = picked_numbers.into_iter().collect();

    let common_numbers: HashSet<_> = pn.intersection(&wn).collect();
    
    if common_numbers.len() == 0{
        0
    }
    else{
        2_i32.pow(common_numbers.len() as u32 - 1)
    }
}