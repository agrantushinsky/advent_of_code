use std::io;

fn solve_part_1(input: &String) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let nums: Vec<u32> = line.chars().filter(|c| c.is_ascii_digit() && *c != '0').map(|c| c.to_digit(10).unwrap()).collect();
        
        sum += nums.first().unwrap() * 10 + nums.last().unwrap();
    }

    sum
}

fn main() {
    let mut buffer = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buffer) {
        if n == 0 { 
            break;
        }
    }

    println!("part 1: {:?}", solve_part_1(&buffer));
}