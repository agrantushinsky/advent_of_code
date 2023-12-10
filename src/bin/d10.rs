use std::{io, str::FromStr, fmt::Debug};

fn split_parse<T>(input: &str) -> Vec<T> where T: FromStr, T::Err: Debug {
    return input
        .split_ascii_whitespace()
        .map(|data| data.parse::<T>().unwrap())
        .collect();
}

fn solve(input: &String) -> (i32, i32) {

    (0, 0)
}

fn main() {
    let mut buffer = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buffer) {
        if n == 0 { 
            break;
        }
    }

    println!("{:?}", solve(&buffer));
}