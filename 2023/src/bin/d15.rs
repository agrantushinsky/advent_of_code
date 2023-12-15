use std::{io::{self, Read}, fs::{self, File}};

fn hash(str: &str) -> u32 {
    str.chars().fold(0, |current_value, character| {
        let mut current = current_value + character as u32;
        current *= 17;
        current %= 256;
        current
    })
}

enum OperationMode {
    Minus,
    Plus(u32)
}

struct Operation {
    label: u32,
    mode: OperationMode
}

fn solve(input: &String) -> u32 {
    let mut boxes: [Vec<u32>; 256] = vec![Vec::new(); 256].try_into().unwrap();

        //.fold(0 as u32, |sum, current| sum + hash(current))
    let lens_operations = input
        .trim()
        .split(',')
        .map(|str| {
            if let Some(pos) = str.find('-') {
                Operation {
                    label: hash(&str[0..pos]),
                    mode: OperationMode::Minus
                }
            } else if let Some(pos) = str.find('=') {
                Operation {
                    label = hash(&str[0..pos]),
                    mode: OperationMode::Plus() // TODO
                }
            } else {
                panic!("bad input");
            }
        });
}

fn main() {
    let mut file = File::open("d15.txt");
    let mut buffer = String::new();
    file.unwrap().read_to_string(&mut buffer).expect("read file");

    println!("{:?}", solve(&buffer));
}
