use std::io;
use itertools::Itertools;

fn solve(input: &String) -> (i32, i32) {
    let lines: Vec<&str> = input.lines().collect();
    let groups = lines.iter().group_by(|line| line.is_empty());
    let sums = groups.into_iter()
        .filter(|(empty, _)| !empty)
        .map(|(_, group)| group.fold(0, |a, curr| a + curr.parse::<i32>().unwrap()))
        .sorted().rev();

    (sums.clone().next().unwrap(), sums.take(3).sum())
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