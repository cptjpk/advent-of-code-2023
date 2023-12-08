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

struct Symbol {
    value: char,
    x: i32,
    y: i32,
    adj: i32,
}

fn main() {
    let mut file = File::open("./input.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let mut numbers: Vec<Number> = Vec::new();
    let mut sum: u32;

    let lines: Vec<&str> = contents.split('\n').collect();

    sum = part_one(&lines, &mut numbers);
    println!("Part One Sum: {}", sum);

    sum = part_two(&lines, &mut numbers);
    println!("Part Two Sum: {}", sum);
}

fn part_one(lines: &Vec<&str>, numbers: &mut Vec<Number>) -> u32{

    let mut sum: u32 = 0;

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
                    if x == line.len() - 1 {
                        numbers.push(Number {
                            value: t_num,
                            x: (x as i32 - t_num.to_string().len() as i32),
                            y: y as i32,
                            length: t_num.to_string().len() as i32,
                            found: false,
                        });
                        t_num = -1;
                    }
                }
                continue;
            }
            else if !c.is_numeric() && t_num != -1{
                numbers.push(Number {
                    value: t_num,
                    x: (x as i32 - t_num.to_string().len() as i32),
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
                for dx in 0..3{
                    for dy in 0..3{
                        let cx = x + dx - 1;
                        let cy = y + dy - 1;

                        for num in numbers.iter_mut(){
                            if num.y == cy as i32{
                                let pos_x = num.x .. num.x + num.length as i32;
                                if pos_x.contains(&(cx as i32)) && !num.found{
                                    num.found = true;
                                    sum += num.value as u32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    sum
}

fn part_two(lines: &Vec<&str>, numbers: &mut Vec<Number>) -> u32{

    let mut sum: u32 = 0;

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
                    if x == line.len() - 1 {
                        numbers.push(Number {
                            value: t_num,
                            x: (x as i32 - t_num.to_string().len() as i32),
                            y: y as i32,
                            length: t_num.to_string().len() as i32,
                            found: false,
                        });
                        t_num = -1;
                    }
                }
                continue;
            }
            else if !c.is_numeric() && t_num != -1{
                numbers.push(Number {
                    value: t_num,
                    x: (x as i32 - t_num.to_string().len() as i32),
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
            if c == '*' {
                let mut adj = 0;
                let mut adj_s = 0;
                for dx in 0..3{
                    for dy in 0..3{
                        let cx = x + dx - 1;
                        let cy = y + dy - 1;

                        for num in numbers.iter_mut(){
                            if num.y == cy as i32 && adj < 3{
                                let pos_x = num.x .. num.x + num.length as i32;
                                if pos_x.contains(&(cx as i32)) && !num.found{
                                    num.found = true;
                                    if adj == 0{
                                        adj_s = num.value;
                                    }
                                    else{
                                        adj_s *= num.value;
                                    }
                                    adj += 1;
                                }
                            }
                        }
                    }
                }
                if adj == 2{
                    sum += adj_s as u32;
                }
            }
        }
    }

    sum
}