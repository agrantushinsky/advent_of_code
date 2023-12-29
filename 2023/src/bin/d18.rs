use std::{io, time::SystemTime, ops::Add, collections::{HashSet, HashMap}};

use itertools::Itertools;

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

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn direction_to_force(direction: Direction) -> Vec2d {
    use Direction::*;
    match direction {
        Up => Vec2d { x: 0, y: -1 },
        Down => Vec2d { x: 0, y: 1 },
        Left => Vec2d { x: -1, y: 0 },
        Right => Vec2d { x: 1, y: 0 },
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct DigPlan {
    direction: Direction,
    distance: i32,
    colour: u64
}

fn solve(input: &String) -> i32 {
    let instructions: Vec<DigPlan> = input
        .lines()
        .map(|line| {
            let distance = *&line[2..line.find('(').unwrap()-1].parse::<i32>().unwrap();
            let colour = u64::from_str_radix(&line[line.find('#').unwrap()+1..line.find(')').unwrap()], 16).unwrap();
            match line.split_at(1).0 {
                "U" => DigPlan { direction: Direction::Up, distance, colour },
                "D" => DigPlan { direction: Direction::Down, distance, colour },
                "L" => DigPlan { direction: Direction::Left, distance, colour },
                "R" => DigPlan { direction: Direction::Right, distance, colour },
                _ => panic!("bad direction")
            }
        }).collect();

    let mut path = HashMap::new();
    let mut position = Vec2d { x: 0, y: 0 };
    let mut upper = position;
    let mut lower = position;
    for ins in instructions {
        for _ in 0..ins.distance {
            path.insert(position.clone(), ins.direction);
            position = position + direction_to_force(ins.direction);

            if position.x > upper.x  {
                upper.x = position.x;
            } 
            if position.y > upper.y {
                upper.y = position.y;
            }
            if position.x < lower.x  {
                lower.x = position.x;
            } 
            if position.y < lower.y {
                lower.y = position.y;
            }
        }
    }

    for y in lower.y..=upper.y {
        for x in lower.x..=upper.x {
            match path.get(&Vec2d { x, y }) {
                //Some(_) => print!("#"),
                Some(d) => {
                    match d {
                        Direction::Up => print!("U"),
                        Direction::Down => print!("D"),
                        Direction::Left => print!("L"),
                        Direction::Right => print!("R"),
                    }
                }
                None => print!(".")
            }
        }
        println!();
    }

    let mut area = path.len() as i32; // start with the perimeter

    for y in lower.y..=upper.y {
        let mut walls = path.iter()
            .filter(|(p, _)| p.y == y)
            .sorted_by(|(a, _), (b, _)| a.x.cmp(&b.x))
            .peekable();

        let mut inside = true; // TODO: Here
        while let Some((left, ldir)) = walls.next() {
            if walls.peek().is_none() { break; }
            let (right, rdir) = walls.peek().unwrap();
            
            let dist = right.x - left.x;
            if dist == 1 { continue; }
            area += dist - 1;
            _ = walls.next();
            //println!("Adding {} (r: {} - l: {})", dist - 1, right.x, left.x);
        }
    }

    area
}
// 18957 low

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
