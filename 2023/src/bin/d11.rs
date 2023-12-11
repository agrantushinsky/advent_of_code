use std::io;

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Galaxy,
    Empty
}

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<Value>>,
}

impl Universe {
    fn new(input: &String) -> Self {
        use Value::*;
        
        let grid = 
            input.lines()
            .map(|line| line.chars().map(|c| {
                match c {
                    '.' => Empty,
                    '#' => Galaxy,
                    _ => unimplemented!("value not found")
                }
            }).collect()).collect();
        
        Universe {
            grid
        }
    }

    fn expand(&mut self) {
        let mut height = self.grid.len();
        let mut y = 0;
        while y < height {
            let row = self.grid.get(y).unwrap();
            if row.iter().all(|c| *c == Value::Empty) {
                self.grid.insert(y, row.to_vec());
                height += 1;
                y += 1;
            }
            y += 1;
        } 

        let mut width = self.grid[0].len();
        let mut x = 0;
        while x < width {
            if self.grid.iter().all(|r| r[x] == Value::Empty) {
                self.grid.iter_mut().for_each(|row| row.insert(x, Value::Empty));
                width += 1;
                x += 1;
            }
            x += 1;
        }
    }

    fn get_galaxies(&self) -> Vec<(usize, usize)> {
        let mut points = vec![];

        for (y, row) in self.grid.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value == Value::Galaxy {
                    points.push((x, y));
                }
            }
        }

        points
    }
}

impl ToString for Universe {
    fn to_string(&self) -> String {
        use Value::*;
        // This is probably awful. Oh well.
        let out = self.grid.iter().
            map(|line| line.iter().fold(String::new(), |a, v| {
                a + match v {
                    Empty => ".",
                    Galaxy => "#"
                }
            })).fold(String::new(), |a, l| {
                a + &l + "\n"
            });
        out
    }
}

fn calculate_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let dx = a.0 as i32 - b.0 as i32;
    let dy = a.1 as i32 - b.1 as i32;

    dx.abs() as usize + dy.abs() as usize
}

fn solve(input: &String) -> (usize, usize) {
    let mut universe = Universe::new(input);
    universe.expand();

    let mut pairs = vec![];

    let galaxies = universe.get_galaxies();
    for (i, point) in galaxies.iter().enumerate() {
        galaxies[i + 1..].iter().for_each(|point2| {
            pairs.push((point, point2));
        })
    }

    //println!("{:?}", pairs);
    //println!("{} len", pairs.len());

    let distance_sum = pairs.iter().fold(0, |a, (p1, p2)| {
        a + calculate_distance(*p1, *p2)
    });

    (distance_sum, 0)
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
