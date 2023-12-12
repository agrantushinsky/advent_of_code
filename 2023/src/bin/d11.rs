use std::{io, vec};

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Galaxy,
    Empty
}

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<Value>>,
    expanded_rows: Vec<usize>,
    expanded_columns: Vec<usize>,
    expansion_factor: usize
}

impl Universe {
    fn new(input: &String, expansion_factor: usize) -> Self {
        use Value::*;
        
        let grid: Vec<Vec<Value>> = 
            input.lines()
            .map(|line| line.chars().map(|c| {
                match c {
                    '.' => Empty,
                    '#' => Galaxy,
                    _ => unimplemented!("value not found")
                }
            }).collect()).collect();

        let mut expanded_rows: Vec<usize> = vec![];
        let mut expanded_columns: Vec<usize> = vec![];

        for (y, row) in grid.iter().enumerate() {
            if row.iter().all(|c| *c == Value::Empty) {
                expanded_rows.push(y)
            }
        } 

        for x in 0..grid[0].len() {
            if grid.iter().all(|r| r[x] == Value::Empty) {
                expanded_columns.push(x);
            }
        }

        Universe {
            grid,
            expanded_rows,
            expanded_columns,
            expansion_factor
        }
    }

    fn get_real_point(&self, point: (usize, usize)) -> (usize, usize) {
        let convert = |o_coordinate: usize, exps: &Vec<usize>| -> usize {
            let applicable_expansions: Vec<_> = exps.iter()
                .filter(|ex| o_coordinate > **ex).collect();

            let new_coordinate = o_coordinate + (applicable_expansions.len() * self.expansion_factor) - applicable_expansions.len();

            new_coordinate
        };
        
        (convert(point.0, &self.expanded_columns), convert(point.1, &self.expanded_rows))
    }

    fn get_galaxies(&self) -> Vec<(usize, usize)> {
        let mut points = vec![];

        for (y, row) in self.grid.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value == Value::Galaxy {
                    points.push(self.get_real_point((x, y)));
                }
            }
        }

        points
    }
}

fn calculate_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let dx = a.0 as i64 - b.0 as i64;
    let dy = a.1 as i64 - b.1 as i64;

    dx.abs() as usize + dy.abs() as usize
}

fn solve(input: &String) -> usize {
    let universe = Universe::new(input, 1000000);

    let mut pairs = vec![];

    let galaxies = universe.get_galaxies();
    for (i, point) in galaxies.iter().enumerate() {
        galaxies[i + 1..].iter().for_each(|point2| {
            pairs.push((point, point2));
        })
    }

    let distance_sum = pairs.iter().fold(0, |a, (p1, p2)| {
        a + calculate_distance(*p1, *p2)
    });

    distance_sum
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
