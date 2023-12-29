use std::{io, time::SystemTime, ops::Add};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Vec2d {
    x: i32,
    y: i32
}

impl Add for Vec2d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[derive(Debug)]
struct DigPlan {
    direction: Vec2d,
    colour: u64
}

fn solve(input: &String) -> u32 {
    let instructions: Vec<DigPlan> = input
        .lines()
        .map(|line| {
            let distance = *&line[2..line.find('(').unwrap()-1].parse::<i32>().unwrap();
            let colour = u64::from_str_radix(&line[line.find('#').unwrap()+1..line.find(')').unwrap()], 16).unwrap();
            match line.split_at(1).0 {
                "U" => DigPlan { direction: Vec2d { x: 0, y: -distance }, colour },
                "D" => DigPlan { direction: Vec2d { x: 0, y: distance }, colour },
                "L" => DigPlan { direction: Vec2d { x: -distance, y: 0 }, colour },
                "R" => DigPlan { direction: Vec2d { x: distance, y: 0 }, colour },
                _ => panic!("bad direction")
            }
        }).collect();

    dbg!(instructions);

    unimplemented!();
}

fn main() {
    let mut buffer = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buffer) {
        if n == 0 { 
            break;
        }
    }

    let start = SystemTime::now();
    println!("{:?}", solve(&buffer));
    println!("solved in {}ms", SystemTime::now().duration_since(start).unwrap().as_millis())
}
