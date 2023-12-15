use std::{io, collections::VecDeque};

#[derive(Debug, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

#[derive(Debug)]
struct Record {
    damaged_ranges: VecDeque<u8>,
    springs: Vec<Spring>
}

impl Record {
    fn new(input: &str) -> Self {
        use Spring::*;

        let splits: Vec<&str> = input.split_ascii_whitespace().collect();
        let springs = splits[0].chars().map(|c| {
            match c {
                '.' => Operational,
                '#' => Damaged,
                '?' => Unknown,
                _ => panic!("bad input")
            }
        }).collect();
        let damaged_ranges = splits[1].split(',').map(|num| num.parse::<u8>().unwrap()).collect();
        Record {
            damaged_ranges,
            springs
        }
    }
}

fn get_records(input: &String) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            Record::new(line)
        })
        .collect()
}

fn does_satisfy_ranges(record: &Record) -> bool {
    use Spring::*;


    let mut current_damaged: u8 = 0;
    let mut spring_iter = record.springs.iter().peekable();
    let mut damaged_ranges = record.damaged_ranges.clone();

    while let Some(spring) = spring_iter.next() {
        if (*spring == Operational && current_damaged != 0) || spring_iter.peek().is_none() {
            let range = damaged_ranges.pop_front();
            if range.is_some() && current_damaged != range.unwrap() {
                return false;
            } else if current_damaged > 0 {
                return false;
            }
        } else {
            current_damaged += 1;
        }
    }

    record.damaged_ranges.is_empty()
}

fn solve(records: &mut Vec<Record>) -> u32 {
    let mut sum = 0;
    for record in records {
        // naive? no.
        let unknowns = record.springs.iter().filter(|s| **s == Spring::Unknown);

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

    let mut records = get_records(&buffer);

    println!("{:?}", solve(&mut records));
}
