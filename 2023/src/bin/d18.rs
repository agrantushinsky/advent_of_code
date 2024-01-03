use std::{io, time::SystemTime, ops::Add};


#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Vec2d {
    x: i64,
    y: i64
}

impl Add for Vec2d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

fn solve(input: &String) -> i64 {
    // Part 1:
    /*
    let instructions: Vec<Vec2d> = input
        .lines()
        .map(|line| {
            let distance = *&line[2..line.find('(').unwrap()-1].parse::<i64>().unwrap();
            //let colour = u64::from_str_radix(&line[line.find('#').unwrap()+1..line.find(')').unwrap()], 16).unwrap();
            match line.split_at(1).0 {
                "U" => Vec2d { x: 0, y: -distance },
                "D" => Vec2d { x: 0, y: distance },
                "L" => Vec2d { x: -distance, y: 0 },
                "R" => Vec2d { x: distance, y: 0 },
                _ => panic!("bad direction")
            }
        }).collect(); */

    // Part 2:
    let instructions: Vec<Vec2d> = input
        .lines()
        .map(|line| {
            let distance = i64::from_str_radix(&line[line.find('#').unwrap()+1..line.find(')').unwrap()-1], 16).unwrap();
            let direction = &line[line.find(')').unwrap()-1..line.find(')').unwrap()];
            match direction {
                "3" => Vec2d { x: 0, y: -distance },
                "1" => Vec2d { x: 0, y: distance },
                "2" => Vec2d { x: -distance, y: 0 },
                "0" => Vec2d { x: distance, y: 0 },
                _ => panic!("bad direction")
            }
        }).collect();


    let mut vertices = Vec::new();
    let mut position = Vec2d { x: 0, y: 0 };
    let mut perimeter = 0f64;

    for ins in instructions {
        perimeter += (ins.x.abs() + ins.y.abs()) as f64;

        position = position + ins;
        vertices.push(position.clone());
    }

    dbg!(&vertices);

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut shoelace_summation = 0f64;
    let num_vertices = vertices.len();
    for i in 0..num_vertices {
         // A of i
         shoelace_summation += 0.5f64 * ((vertices[i].y + vertices[(i + 1) % num_vertices].y) * (vertices[i].x - vertices[(i + 1) % num_vertices].x)) as f64;
    }

    dbg!(&perimeter);
    dbg!(&shoelace_summation);

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    (shoelace_summation + (perimeter / 2f64)) as i64 + 1
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
