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
    position: Vec2d,
    from_dir: Vec2d,
    straight: u32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct State {
    node: Node,
    cost: u32,
}

// Min-heap implementation
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_edges(vertices: &HashMap<Vec2d, u8>, current: &Node, min_steps: u32, max_steps: u32) -> Vec<(Node, u32)> {
    let adj_pos_offsets = vec![
        Vec2d { x: -1, y: 0 },
        Vec2d { x: 1, y: 0 },
        Vec2d { x: 0, y: -1 },
        Vec2d { x: 0, y: 1 },
    ];

    let edges: Vec<(Node, u32)> = adj_pos_offsets.into_iter().filter_map(|offset| { 
        let adj_edge = current.position + offset;
        match vertices.get(&adj_edge) {
            Some(cost) => { 
                if current.from_dir == (Vec2d { x: offset.x * -1, y: offset.y * -1 }) {
                    None
                } else if current.from_dir != offset && current.straight >= min_steps {
                    Some((Node { position: adj_edge, from_dir: offset, straight: 1 }, *cost as u32 ))
                } else if current.from_dir == offset && current.straight < max_steps {
                    Some((Node { position: adj_edge, from_dir: offset, straight: current.straight + 1 }, *cost as u32))
                } else {
                    None
                }
            },
            None => None
        }
    }).into_iter().collect();
    edges
}

fn dijkstras(vertices: &HashMap<Vec2d, u8>, start: Vec2d, end: Vec2d, min_steps: u32, max_steps: u32) -> Option<u32> {
    let mut dist = HashMap::<Node, u32>::new();

    let s1 = Node { position: start, from_dir: Vec2d { x: 1, y: 0 }, straight: 0 };
    let s2 = Node { position: start, from_dir: Vec2d { x: 0, y: 1 }, straight: 0 };

    dist.insert(s1.clone(), 0);
    dist.insert(s2.clone(), 0);

    let mut heap = BinaryHeap::new();
    heap.push(State { node: s1, cost: 0 });
    heap.push(State { node: s2, cost: 0 });

    while let Some(State{ node, cost }) = heap.pop() {
        if node.position == end {
            return Some(cost);
        }

        for (adjacent, adjacent_cost) in get_edges(vertices, &node, min_steps, max_steps) {
            let new_cost = cost + adjacent_cost;

            if !dist.contains_key(&adjacent) || new_cost < dist[&adjacent] {
                heap.push(State { cost: new_cost, node: adjacent });
                dist.insert(adjacent, new_cost);
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

fn solve(input: &String) -> (u32, u32) {
    let map = parse_map(input);

    let end = map.clone().into_iter().max_by_key(|p| p.0.x + p.0.y).unwrap().0;

    (
        dijkstras(&map, Vec2d { x: 0, y: 0 }, end, 1, 3).unwrap(),
        dijkstras(&map, Vec2d { x: 0, y: 0 }, end, 4, 10).unwrap()
    )
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
