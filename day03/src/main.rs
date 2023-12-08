use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone, Debug)]
struct Number {
    value: i32,
    x: i32,
    y: i32,
    length: i32,
    found: bool,
}
fn main() {
    let mut file = File::open("./src/sample.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let mut numbers: Vec<Number> = Vec::new();
    let mut sum = 0;

    let lines: Vec<&str> = contents.split('\n').collect();

    for (y, line) in lines.iter().enumerate() {
        // TODO: Process each line here
        let mut t_num = -1;
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if t_num == -1 {
                    t_num = c.to_digit(10).unwrap() as i32;
                }
                else{
                    t_num = t_num * 10 + c.to_digit(10).unwrap() as i32;
                }
                continue;
            }
            else if !c.is_numeric() && t_num != -1 {
                numbers.push(Number {
                    value: t_num,
                    x: x as i32,
                    y: y as i32,
                    length: t_num.to_string().len() as i32,
                    found: false,
                });
                t_num = -1;
            }
        }
    }

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && !c.is_numeric() {
                println!("Symbol '{}' found at ({}, {})", c, x, y);
                for dx in 0..2{
                    for dy in 0..2{
                        let cx = x + dx - 1;
                        let cy = y + dy - 1;
                        println!("Checking ({}, {})", cx, cy);  
                        
                        for num in numbers.iter_mut(){
                            if num.y == cy as i32{
                                let pos_x = num.x .. num.x + num.length as i32;
                                // println!("{}: {:?}", num.value, pos_x);
                                if pos_x.contains(&(cx as i32)) && !num.found{
                                    println!("Found {}", num.value);
                                    num.found = true;
                                    sum += num.value;
                                    continue;
                                }
                            }
                        }
                    }
                }
                println!("-----------------");
            }
        }
    }
    

    println!("Sum: {}", sum);
}