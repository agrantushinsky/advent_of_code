use std::{io, collections::{HashSet, HashMap, hash_map::DefaultHasher}, ops::{Add, Sub}, time::SystemTime, hash::{Hash, Hasher}};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Vec2d {
    x: i16,
    y: i16
}

impl Add for Vec2d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2d {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Debug)]
struct Platform {
    rounded: HashSet<Vec2d>,
    squared: HashSet<Vec2d>,
    dimensions: Vec2d
}

// Hmmmm...
impl Platform {
    fn new() -> Self {
        Platform {
            rounded: HashSet::new(),
            squared: HashSet::new(),
            dimensions: Vec2d { x: 0, y: 0 }
        }
    }

    fn grow_platform(&mut self, pos: &Vec2d) {
        if pos.x > self.dimensions.x {
            self.dimensions.x = pos.x;
        }
        if pos.y > self.dimensions.y {
            self.dimensions.y = pos.y;
        }
    }

    fn build_platform(input: &String) -> Self {
        let mut platform = Platform::new();
        for (y, row) in input.lines().enumerate() {
            for (x, char) in row.chars().enumerate() {
                let pos = Vec2d { x: x as i16, y: y as i16 };
                platform.grow_platform(&pos);
                match char {
                    'O' => {
                        platform.rounded.insert(pos);
                    }
                    '#' => {
                        platform.squared.insert(pos);
                    }
                    '.' => {}
                    _ => panic!("bad input")
                }
            }
        }

        platform
    }

    fn is_valid_pos(&self, pos: &Vec2d) -> bool {
        pos.x <= self.dimensions.x &&
            pos.y <= self.dimensions.y &&
            pos.x >= 0 &&
            pos.y >= 0 &&
            !self.rounded.contains(pos) &&
            !self.squared.contains(pos)
    }

    fn apply_tilt(&mut self, direction: &Vec2d) {
        let mut changes = true;
        while changes {
            changes = false;

            // apply a tick of direction
            for pos in self.rounded.clone().into_iter() {
                let new_pos = pos + *direction;
                if self.is_valid_pos(&new_pos) {
                    self.rounded.remove(&pos);
                    self.rounded.insert(new_pos);
                    changes = true;
                    continue
                }
            }
        }
    }

    fn calculate_load(&self) -> i32 {
        self.rounded
            .iter()
            .fold(0, |load: i32, pos: &Vec2d| {
                load + self.dimensions.y as i32 + 1 - pos.y as i32
            })
    }
}


fn solve(input: &String) -> i32 {
    let mut platform = Platform::build_platform(input);

    // apply tilt with the North direction
    let cycles = &vec![
        Vec2d { x: 0, y: -1 },
        Vec2d { x: -1, y: 0 },
        Vec2d { x: 0, y: 1 },
        Vec2d { x: 1, y: 0 },
    ];

    let mut arrangements = HashMap::<u64, i32>::new();

    let max_cycles = 1000000000;
    let mut cycle_len: Option<i32> = None;
    let mut i = 0;
    while i < max_cycles {
        for direction in cycles {
            platform.apply_tilt(direction);
        }

        if cycle_len.is_none() {
            let mut hash = DefaultHasher::new();
            let rounded: Vec<&Vec2d> = platform.rounded.iter().collect();
            rounded.hash(&mut hash);

            let arragement_hash = hash.finish();
            if arrangements.contains_key(&arragement_hash) {
                cycle_len = Some(i - arrangements.get(&arragement_hash).unwrap());
                println!("cycle len {}", cycle_len.unwrap());
                println!("cycle start {}", arrangements.get(&arragement_hash).unwrap());
                println!("cycle end {}", i);
                let cycle_skip = cycle_len.unwrap() * ((max_cycles - i) / cycle_len.unwrap());
                i += cycle_skip;

                println!("i skipped {} cycles", cycle_skip);
                println!("{} cycles remaining", max_cycles - i);
            } else{
                arrangements.insert(arragement_hash, i);
            }
        }
        i += 1;
    }

    let load = platform.calculate_load();

    load
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
