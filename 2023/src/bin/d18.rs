use core::num;
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
    Right,
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
    let mut vertices = Vec::new();
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
        vertices.push(position.clone());
    }

    dbg!(&vertices);

    let perimeter = path.len() as f32;

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut shoelace_summation = 0f32;
    let num_vertices = vertices.len();
    for i in 0..num_vertices {
         // A of i
         shoelace_summation += 0.5f32 * ((vertices[i].y + vertices[(i + 1) % num_vertices].y) * (vertices[i].x - vertices[(i + 1) % num_vertices].x)) as f32;
    }

    dbg!(&perimeter);
    dbg!(&shoelace_summation);

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    (shoelace_summation + (perimeter / 2f32)) as i32 + 1
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
