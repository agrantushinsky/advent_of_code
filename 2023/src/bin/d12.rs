use std::io;

#[derive(Debug)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

#[derive(Debug)]
struct Record {
    damaged_ranges: Vec<u32>,
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
        let damaged_ranges = splits[1].split(',').map(|num| num.parse::<u32>().unwrap()).collect();
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

fn solve(records: &Vec<Record>) {
}

fn main() {
    let mut buffer = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buffer) {
        if n == 0 { 
            break;
        }
    }

    let records = get_records(&buffer);

    println!("{:?}", solve(&records));
}
