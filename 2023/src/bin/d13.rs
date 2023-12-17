use std::{io, vec};

use itertools::Itertools;

fn evaluate_reflections(map: &Map) -> u32 {
    let terrain = &map.terrain;

    // rows
    'outer: for y in 1..terrain.len() {
        let y_range = std::cmp::min(y, terrain.len() - y);
        for offset in 0..y_range {
            if terrain[y + offset] != terrain[y - offset - 1] {
                continue 'outer;
            }
        }
        return y as u32 * 100
    }

    // columns
    'outer: for x in 1..terrain[0].len() {
        let x_range = std::cmp::min(x, terrain[0].len() - x);
        for offset in 0..x_range {
            for row in terrain {
                if row.as_bytes()[x + offset] != row.as_bytes()[x - offset - 1] {
                    continue 'outer;
                }
            }
        }
        return x as u32
    }

    0
}

#[derive(Debug)]
struct Map<'a> {
    terrain: Vec<&'a str>
}

fn do_cartography<'a>(input: &'a String) -> Vec<Map<'a>> {
    let mut lines_iter = input.lines().peekable();

    let mut maps: Vec<Map> = vec![];

    while lines_iter.peek().is_some() {
        let mut terrain: Vec<&'a str> = vec![];
        let mut peek = lines_iter.next();
        while !peek.is_none() && !peek.unwrap().is_empty() {
            terrain.push(peek.unwrap());
            peek = lines_iter.next();
        }
        maps.push(Map { terrain });
    }

    maps
} 

fn solve(input: &String) -> u32 {
    let maps = do_cartography(input);

    let sum = maps.iter()
        .map(|map| {
            evaluate_reflections(map)
        })
        .sum::<u32>();

    sum
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
