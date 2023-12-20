use std::{io::Read, time::SystemTime, ops::Add, collections::{HashMap, HashSet, VecDeque}, vec, fs::File};

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

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug)]
enum ReflectorType {
    Vertical,
    Horizontal,
    Slash,
    Dash
}

struct Reflector {
    kind: ReflectorType,
    //reflecting: Vec<Direction>
}

impl Reflector {
    fn reflect(&self, from: Direction) -> Vec<Direction> {
        use Direction::*;
        use ReflectorType::*;
        match self.kind {
            Vertical => {
                match from {
                    Left | Right => vec![ Up, Down ],
                    _ => vec![ from ]
                }
            }
            Horizontal => {
                match from {
                    Up | Down => vec![ Left, Right ],
                    _ => vec![ from ]
                }
            }
            Slash => {
                match from {
                    Up => vec![ Right ],
                    Down => vec![ Left ],
                    Left => vec![ Down ],
                    Right => vec![ Up ]
                }
            }
            Dash => {
                match from {
                    Up => vec![ Left ],
                    Down => vec![ Right ],
                    Left => vec![ Up ],
                    Right => vec![ Down ]
                }
            }
        }
    }
}

struct Puzzle {
    reflectors: HashMap<Vec2d, Reflector>,
    size: Vec2d
}

#[derive(Debug)]
struct Beam {
    pos: Vec2d,
    dir: Direction
}

impl Beam {
    fn new(pos: Vec2d, dir: Direction) -> Self {
        Beam {
            pos, dir
        }
    }
}

impl Puzzle {
    fn grow(&mut self, pos: &Vec2d) {
        if pos.x > self.size.x {
            self.size.x = pos.x;
        }
        if pos.y > self.size.y {
            self.size.y = pos.y;
        }
    }

    fn new(input: &String) -> Self {
        let mut puzzle = Puzzle { 
            reflectors: HashMap::new(), 
            size: Vec2d { x: 0, y: 0 }
        };

        for (y, row) in input.lines().enumerate() {
            for (x, tile) in row.chars().enumerate() {
                let pos = Vec2d { x: x as i32, y: y as i32 };
                puzzle.grow(&pos);
                if let Some(kind) = match tile {
                    '-' => Some(ReflectorType::Horizontal),
                    '|' => Some(ReflectorType::Vertical),
                    '\\' => Some(ReflectorType::Dash),
                    '/' => Some(ReflectorType::Slash),
                    _ => None
                } {
                    puzzle.reflectors.insert(pos, Reflector { kind/*, reflecting: Vec::new() */ });
                }
            }
        }
        puzzle
    }
    fn is_valid_pos(&self, pos: &Vec2d) -> bool {
        pos.x <= self.size.x &&
            pos.y <= self.size.y &&
            pos.x >= 0 &&
            pos.y >= 0
    }

    fn simulate_beam(&self, start_pos: Vec2d, start_dir: Direction) -> HashSet<Vec2d> {
        let mut energized = HashSet::<Vec2d>::new();
        let mut beam_queue = VecDeque::from([Beam::new(start_pos, start_dir)]);
        let mut visited = HashMap::<Vec2d, Vec<Direction>>::new();

        while !beam_queue.is_empty() {
            let curr = beam_queue.pop_front().unwrap();
            energized.insert(curr.pos);
            let next_pos = curr.pos + direction_to_force(curr.dir);
            if self.is_valid_pos(&next_pos) {
                if let Some(tile) = self.reflectors.get(&next_pos) {
                    let tile_visits = visited.get_mut(&curr.pos);
                    if tile_visits.is_some() {
                        let visits = tile_visits.unwrap();
                        if visits.contains(&curr.dir) {
                            continue;
                        }
                        visits.push(curr.dir);
                    } else {
                        visited.insert(curr.pos.clone(), Vec::new());
                    }

                    for reflected_dir in tile.reflect(curr.dir) {
                        beam_queue.push_back(Beam::new(next_pos, reflected_dir));
                    }
                } else {
                    beam_queue.push_back(Beam::new(next_pos, curr.dir));
                }
            }
        }
        energized
    }
}

fn _print_energized(puzzle: &Puzzle, set: &HashSet<Vec2d>) {
    for y in 0..puzzle.size.y + 1 {
        for x in 0..puzzle.size.x + 1 {
            if set.contains(&Vec2d { x: x, y: y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve_part1(puzzle: &Puzzle) -> usize {
    let energized = puzzle.simulate_beam(Vec2d { x: -1, y: 0 }, Direction::Right);

    energized.len() - 1
}

fn solve_part2(puzzle: &Puzzle) -> usize {
    let mut starts = Vec::<(Vec2d, Direction)>::new();
    for x in 0..puzzle.size.x {
        starts.push((Vec2d { x: x, y: -1 }, Direction::Down));
        starts.push((Vec2d { x: x, y: puzzle.size.y + 1 }, Direction::Up));
    }
    for y in 0..puzzle.size.y {
        starts.push((Vec2d { x: -1, y: y }, Direction::Right));
        starts.push((Vec2d { x: puzzle.size.x + 1, y: y }, Direction::Left));
    }

    let mut best_energized = 0;

    for start in starts {
        best_energized = std::cmp::max(best_energized, 
            puzzle.simulate_beam(start.0, start.1).len() - 1);
    }
    
    best_energized
}

fn solve_file(path: &str) {
    let file = File::open(path);
    let mut buffer = String::new();
    file.unwrap().read_to_string(&mut buffer).expect("read file");

    let puzzle = Puzzle::new(&buffer);

    let start = SystemTime::now();
    println!("part 1 solution ({}): {:?}", path, solve_part1(&puzzle));
    println!("solved in {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());

    let start = SystemTime::now();
    println!("part 2 solution ({}): {:?}", path, solve_part2(&puzzle));
    println!("solved in {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
}

fn main() {
    solve_file("d16_sample.txt");
    solve_file("d16.txt");
}
