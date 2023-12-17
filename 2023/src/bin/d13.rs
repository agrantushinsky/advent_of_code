use std::{io, vec};

fn evaluate_reflections(map: &Map) -> u32 {
    let terrain = &map.terrain;

    // every mirror has at least one smudge
    // find the single smudge, as calculate the score based on that I guess...

    // rows
    for y in 1..terrain.len() {
        let y_range = std::cmp::min(y, terrain.len() - y);

        let mut smudges = 0;
        for offset in 0..y_range {
            let r1 = terrain[y + offset];
            let r2 = terrain[y - offset - 1];

            for x in 0..r1.len() {
                if r1.as_bytes()[x] != r2.as_bytes()[x] {
                    smudges += 1;
                }
            }
        }
        if smudges == 1 {
            return y as u32 * 100
        }
    }

    // columns
    for x in 1..terrain[0].len() {
        let x_range = std::cmp::min(x, terrain[0].len() - x);

        let mut smudges = 0;
        for offset in 0..x_range {
            for row in terrain {
                if row.as_bytes()[x + offset] != row.as_bytes()[x - offset - 1] {
                    smudges += 1;
                }
            }
        }
        if smudges == 1 {
            return x as u32
        }
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
