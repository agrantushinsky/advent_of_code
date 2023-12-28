use std::{io, time::SystemTime, collections::{BinaryHeap, HashMap}, ops::{Add, Sub}};

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

impl Sub for Vec2d {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Node {
    cost: u32,
    position: Vec2d,
    from_dir: Vec2d,
    straight: u32,
}

// Min-heap implementation
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_edges(vertices: &HashMap<Vec2d, u8>, position: &Vec2d) -> Vec<(Vec2d, u32, Vec2d)> {
    let adj_pos_offsets = vec![
        Vec2d { x: -1, y: 0 },
        Vec2d { x: 1, y: 0 },
        Vec2d { x: 0, y: -1 },
        Vec2d { x: 0, y: 1 },
    ];

    let edges: Vec<(Vec2d, u32, Vec2d)> = adj_pos_offsets.into_iter().filter_map(|pos| { 
        let adj_edge = *position + pos;
        match vertices.get(&adj_edge) {
            Some(cost) => Some((adj_edge, *cost as u32, pos)),
            None => None
        }
    }).into_iter().collect();
    edges
}

fn dijkstras(vertices: &HashMap<Vec2d, u8>, start: Vec2d, end: Vec2d) -> Option<(u32, HashMap<Vec2d, (Vec2d, Vec2d)>)> {
    let mut dist = 
        vertices.into_iter().fold(HashMap::new(), |mut dists, (pos, _)| {
            dists.insert(pos, std::u32::MAX); 
            dists
        });

    let mut prev = HashMap::<Vec2d, (Vec2d, Vec2d)>::new();

    dist.insert(&start, 0);

    let mut heap = BinaryHeap::new();
    heap.push(Node { position: start, cost: 0, from_dir: Vec2d { x: -1, y: 0 }, straight: 0 });

    while let Some(node) = heap.pop() {
        if node.position == end {
            return Some((node.cost, prev));
        }

        if node.cost > dist[&node.position] { 
            continue;
        }

        for (edge_pos, edge_cost, off) in get_edges(vertices, &node.position) {
            let straight = if off == node.from_dir { node.straight } else { 0 };
            if straight >= 3 {
                continue;
            }

            let next = Node { cost: node.cost + edge_cost, position: edge_pos, from_dir: off, straight: straight + 1 };

            let edge_node = dist.get_mut(&next.position).unwrap();
            if next.cost < *edge_node {
                heap.push(next);
                prev.insert(next.position, (node.position, off));
                *edge_node = next.cost;
            }
        }
    }

    None
}

fn parse_map(input: &String) -> HashMap<Vec2d, u8> {
    let mut map = HashMap::new();

    input.lines().enumerate().for_each(
        |(y, row)| { row.char_indices().for_each(
            |(x, byte)| {
                map.insert(Vec2d { x: x as i32, y: y as i32 }, byte.to_digit(10).unwrap() as u8);
            }
        )}
    );

    map
}

fn solve(input: &String) -> u32 {
    let map = parse_map(input);

    let end = map.clone().into_iter().max_by_key(|p| p.0.x + p.0.y).unwrap().0;

    let (heat_loss, paths) = dijkstras(&map, Vec2d { x: 0, y: 0 }, end).unwrap();
    let mut curr = (end, Vec2d { x: 0, y: 0 });
    let mut path = HashMap::new();
    while let Some(next) = paths.get(&curr.0) {
        path.insert(curr.0, curr.1);
        curr = *next;
    }

    for y in 0..(end.y + 1) {
        for x in 0..(end.x + 1) {
            if let Some(dir) = path.get(&Vec2d { x, y }) {
                print!("{}", match dir {
                    Vec2d { x: -1, y: 0 } => "<",
                    Vec2d { x: 1, y: 0 } => ">",
                    Vec2d { x: 0, y: -1 } => "^",
                    Vec2d { x: 0, y: 1 } => "v",
                    _ => "#"
                });
            } else {
                //print!("{}", map.get(&Vec2d { x, y }).unwrap());
                print!(".");
            }
        }
        println!();
    }

    heat_loss
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
