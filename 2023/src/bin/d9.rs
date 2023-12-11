use std::{io, collections::VecDeque};

struct Calibration {
    layers: Vec<VecDeque<i32>>
}

impl Calibration {
    fn generate_layers(&mut self, current: usize) {
        let mut new_layer = VecDeque::new();
        let current_layer = &self.layers[current];
        for (i, num) in current_layer.iter().enumerate() {
            // Surely there is a better way of doing this.
            if i == current_layer.len() - 1 { break; }

            let next_num = self.layers[current][i + 1];

            new_layer.push_back(next_num - num);
        }

        let all_zero = new_layer.iter().all(|x| *x == 0);
        self.layers.push(new_layer);
        
        if !all_zero {
            self.generate_layers(current + 1)
        }
    }

    fn extrapolate(&mut self) -> (i32, i32) {
        // Add the zeroes first:
        self.layers.last_mut().unwrap().push_back(0);
        self.layers.last_mut().unwrap().push_front(0);

        let mut prev_back= 0;
        let mut prev_front = 0;
        for (_, layer) in self.layers.iter_mut().rev().enumerate().skip(1) {
            let back= *layer.back().unwrap();
            let front= *layer.front().unwrap();

            let sum = back + prev_back;
            let diff = front - prev_front;

            layer.push_back(sum);
            layer.push_front(diff);

            prev_back = sum;
            prev_front = diff;
        }

        // Return extrapolated front & back values.
        let first_layer = self.layers.first().unwrap();
        (*first_layer.front().unwrap(), *first_layer.back().unwrap())
    }
}

fn parse_line_as_numbers(line: &str) -> Vec<i32> {
    return line
        .split_ascii_whitespace()
        .map(|num| num.parse::<i32>()
        .unwrap())
        .collect();
}

fn solve(input: &String) -> (i32, i32) {
    let mut calibrations: Vec<Calibration> = Vec::new();

    for line in input.lines() {
        let numbers = parse_line_as_numbers(line);
        let mut cali = Calibration { layers: vec![VecDeque::from(numbers)] };
        cali.generate_layers(0);
        calibrations.push(cali);
    }

    let mut sum_back = 0;
    let mut sum_front = 0;

    for (_, cali) in calibrations.iter_mut().enumerate() {
        let extra = cali.extrapolate();
        sum_front += extra.0;
        sum_back += extra.1;
    }

    (sum_front, sum_back)
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