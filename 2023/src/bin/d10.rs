use std::{io, str::FromStr, fmt::Debug, collections};

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East
        }
    }
}

#[derive(Debug)]
struct Tile {
    char: char,
    connections: Option<[Direction; 2]>,
    explored: bool
}

impl Tile {
    fn new(char: char) -> Self {
        Tile {
            char,
            connections: match char {
                '|' => Some([ Direction::North, Direction::South ]),
                '-' => Some([ Direction::East, Direction::West ]),
                'L' => Some([ Direction::North, Direction::East ]),
                'J' => Some([ Direction::North, Direction::West ]),
                '7' => Some([ Direction::South, Direction::West ]),
                'F' => Some([ Direction::South, Direction::East ]),
                _ => None
            },
            explored: false
        }
    }
}

// integer vector? i see no problem.
#[derive(Copy, Clone)]
struct Vector2D {
    x: i32,
    y: i32
}

impl Vector2D {
    fn new(x: i32, y: i32) -> Vector2D { 
        Vector2D{ x, y} 
    }
}

struct Map {
    grid: Vec<Vec<Tile>>,
    start_pos: Vector2D
}

impl Map {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Tile>> = Vec::new();
        let mut start_pos = Vector2D::new(-1, -1);
        
        for (y, line) in input.lines().enumerate() {
            let mut grid_row = Vec::new();
            for (x, character) in line.chars().enumerate() {
                if character == 'S' {
                    start_pos = Vector2D::new(x as i32, y as i32);
                }
                grid_row.push(Tile::new(character));
            }
            grid.push(grid_row);
        }

        Map {
            grid, start_pos
        }
    }

    fn set_explored(&mut self, pos: &Vector2D) {
        let tile = self.grid.get_mut(pos.y as usize).unwrap().get_mut(pos.x as usize).unwrap();
        tile.explored = true;
    }

    fn get(&mut self, pos: &Vector2D) -> Option<&Tile> {
        if pos.y < 0 || pos.x < 0 { 
            return None; 
        }
        let row = self.grid.get_mut(pos.y as usize);
        if row.is_some() {
            return row.unwrap().get(pos.x as usize);
        }
        None
    }

    fn get_from_direction(&mut self, pos: &Vector2D, direction: Direction) -> Option<&Tile> {
        match direction {
            Direction::North => self.get(&Vector2D::new(pos.x, pos.y - 1)),
            Direction::East => self.get(&Vector2D::new(pos.x + 1, pos.y)),
            Direction::South => self.get(&Vector2D::new(pos.x, pos.y + 1)),
            Direction::West => self.get(&Vector2D::new(pos.x - 1, pos.y))
        }
    }

    fn get_adjacent_tiles(&mut self, pos: &Vector2D) -> [Option<&Tile>; 4] {
        [
            self.get_from_direction(&pos, Direction::North),
            self.get_from_direction(&pos, Direction::East),
            self.get_from_direction(&pos, Direction::South),
            self.get_from_direction(&pos, Direction::West)
        ]
    }

    fn get_start_pos(&self) -> &Vector2D { &self.start_pos }
}

fn split_parse<T>(input: &str) -> Vec<T> where T: FromStr, T::Err: Debug {
    return input
        .split_ascii_whitespace()
        .map(|data| data.parse::<T>().unwrap())
        .collect();
}

struct BFSNode {
    pos: Vector2D, 
    steps: i32
}

fn solve(input: &String) -> (i32, i32) {
    let mut map = Map::new(input);

    let start_pos = map.get_start_pos().clone();
    
    // https://en.wikipedia.org/wiki/Breadth-first_search
    let mut bfs_queue: collections::VecDeque<BFSNode> = collections::VecDeque::new();
    // enqueue the root node
    bfs_queue.push_back(BFSNode { pos: start_pos, steps: 0 });

    while !bfs_queue.is_empty() {
        let node = bfs_queue.pop_front().unwrap();
        let curr_tile = map.get(&node.pos);
        for (_, tile) in map.get_adjacent_tiles(&node.pos).iter_mut().enumerate() {
            if tile.is_some() {
                tile.unwrap().explored = true;
                println!("{:?}", tile.unwrap());
            }
            //if tile.is_some_and(|t| t.connections.is_some_and(|c| c.iter().any(|d| curr_tile.unwrap().connections.unwrap().contains(&d.opposite())))) {

            //}
        }
    }


    (0, 0)
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