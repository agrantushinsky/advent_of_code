use std::{io::Read, fs::File};

use itertools::Itertools;

fn hash(str: &str) -> u32 {
    str.chars().fold(0, |current_value, character| {
        let mut current = current_value + character as u32;
        current *= 17;
        current %= 256;
        current
    })
}

#[derive(Debug, PartialEq)]
enum OperationMode {
    Minus,
    Equals(u32)
}

#[derive(Debug, Clone)]
struct Label<'a> {
    text: &'a str,
    hash: u32,
}

#[derive(Debug)]
struct Operation<'a> {
    label: Label<'a>,
    mode: OperationMode
}

fn solve(input: &String) -> u32 {
    // part 1 sum:
    //.fold(0 as u32, |sum, current| sum + hash(current))

    let lens_operations: Vec<Operation> = input
        .trim()
        .split(',')
        .map(|str| {
            if let Some(pos) = str.find('-') {
                Operation {
                    label: Label {
                        text: &str[0..pos],
                        hash: hash(&str[0..pos]),
                    },
                    mode: OperationMode::Minus
                }
            } else if let Some(pos) = str.find('=') {
                Operation {
                    label: Label {
                        text: &str[0..pos],
                        hash: hash(&str[0..pos]),
                    },
                    mode: OperationMode::Equals(*&str[pos + 1..].parse::<u32>().unwrap())
                }
            } else {
                panic!("bad input");
            }
        }).collect();

    // Apply every operation to the boxes.
    let mut boxes: [Vec<(Label, u32)>; 256] = vec![Vec::new(); 256].try_into().unwrap();
    'op: for operation in lens_operations {
        let bx = boxes.get_mut(operation.label.hash as usize).unwrap();
        match operation.mode {
            OperationMode::Minus => {
                if let Some(label_pos) = bx.iter().find_position(|b| b.0.text == operation.label.text) {
                    bx.remove(label_pos.0);
                }
            }
            OperationMode::Equals(focal_length) => {
                for i in 0..bx.len() {
                    let box_fl = bx.get_mut(i).unwrap();
                    if box_fl.0.text == operation.label.text {
                        box_fl.1 = focal_length;
                        continue 'op;
                    }
                }
                bx.push((operation.label, focal_length));
            }
        }
    }
    
    let mut sum: u32 = 0;
    for (box_num, lenss) in boxes.iter().enumerate() {
        if lenss.is_empty() {
            continue;
        }
        for (slot, lens) in lenss.iter().enumerate() {
            sum += (box_num + 1) as u32 * (slot + 1) as u32 * lens.1;
        }
    }
    
    sum
}

fn solve_file(path: &str) {
    let file = File::open(path);
    let mut buffer = String::new();
    file.unwrap().read_to_string(&mut buffer).expect("read file");

    println!("{:?}", solve(&buffer));
}

fn main() {
    solve_file("d15_sample.txt");
    solve_file("d15.txt");
}
