use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    result,
};

struct Task {
    leftOf: HashMap<i32, Vec<i32>>,
    rightOf: HashMap<i32, Vec<i32>>,
    inputs: Vec<Vec<i32>>,
}

enum ReaderState {
    Rules,
    Inputs,
}

enum Direction {
    Left,
    Right,
}

impl Task {
    fn from_file(path: &Path) -> Task {
        let mut left_of = HashMap::new();
        let mut right_of = HashMap::new();
        let mut inputs = Vec::new();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut state = ReaderState::Rules;
        for line in reader.lines() {
            let line = line.unwrap();
            if line.len() == 0 {
                state = ReaderState::Inputs;
                continue;
            }
            match state {
                ReaderState::Rules => {
                    let parts: Vec<&str> = line.split("|").collect();
                    let left = parts[0].parse::<i32>().unwrap();
                    let right = parts[1].parse::<i32>().unwrap();
                    left_of.entry(left).or_insert(Vec::new()).push(right);
                    right_of.entry(right).or_insert(Vec::new()).push(left);
                }
                ReaderState::Inputs => {
                    let parts: Vec<&str> = line.split(",").collect();
                    let numbers: Vec<i32> =
                        parts.iter().map(|x| x.parse::<i32>().unwrap()).collect();
                    inputs.push(numbers);
                }
            }
        }
        Task {
            leftOf: left_of,
            rightOf: right_of,
            inputs,
        }
    }

    fn is_valid(&self, i: i32, dir: Direction, j: i32) -> bool {
        match dir {
            Direction::Left => {
                if let Some(rights) = self.rightOf.get(&i) {
                    for right in rights {
                        if *right == j {
                            return true;
                        }
                    }
                }
            }
            Direction::Right => {
                if let Some(lefts) = self.leftOf.get(&i) {
                    for left in lefts {
                        if *left == j {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    fn solve_part_one(&self) -> i32 {
        let mut result = 0;
        for input in &self.inputs {
            let mut input_valid = true;
            for i in 0..input.len() {
                let number = input[i];
                let mut valid = true;
                for j in 0..i {
                    valid = valid && self.is_valid(number, Direction::Left, input[j]);
                }
                for j in i + 1..input.len() {
                    valid = valid && self.is_valid(number, Direction::Right, input[j]);
                }
                input_valid = input_valid && valid;
            }
            if input_valid {
                result += input[input.len() / 2];
            }
        }
        result
    }

    fn input_valid(&self, input: &Vec<i32>) -> bool {
        let mut input_valid = true;
        for i in 0..input.len() {
            let number = input[i];
            let mut valid = true;
            for j in 0..i {
                valid = valid && self.is_valid(number, Direction::Left, input[j]);
            }
            for j in i + 1..input.len() {
                valid = valid && self.is_valid(number, Direction::Right, input[j]);
            }
            input_valid = input_valid && valid;
        }
        input_valid
    }

    fn try_sort(&self, mut cur_solution: Vec<i32>, mut open: Vec<i32>) -> Option<Vec<i32>> {
        // If our current solution is empty, we just take the first element from the open list
        if cur_solution.len() == 0 {
            cur_solution.push(open[0]);
            open.remove(0);
            return self.try_sort(cur_solution, open);
        }

        // If the open list is empty, we have a solution
        if open.len() == 0 {
            return Some(cur_solution);
        }

        // We try to insert the first element from the open list at every possible position
        let i = 0;
        // We try to insert the element at every possible position
        for j in 0..cur_solution.len()+1 {
            let mut new_solution = cur_solution.clone();
            new_solution.insert(j, open[i]);
            let mut new_open = open.clone();
            new_open.remove(i);

            println!("Inserting {} at position {} in {:?} it becomes {:?}", open[i], j, cur_solution, new_solution);

            if !self.input_valid(&new_solution) {
                continue;
            }

            if let Some(solution) = self.try_sort(new_solution, new_open) {
                return Some(solution);
            }
        }
        None
    }

    fn solve_part_two(&self) -> i32 {
        let mut incorectly_sorted_inputs = Vec::new();

        for input in &self.inputs {
            if !self.input_valid(input) {
                incorectly_sorted_inputs.push(input.clone());
            }
        }

        let mut result = 0;
        for input in incorectly_sorted_inputs {
            let mut open = input.clone();
            let solution = self.try_sort(Vec::new(), open);
            if let Some(solution) = solution {
                result += solution[solution.len() / 2];
            }
        }
        result
    }
}

fn main() {
    let task = Task::from_file(Path::new("input.txt"));

    println!("Should be true");
    println!(
        "Test 53 right of 47: {}",
        task.is_valid(47, Direction::Right, 53)
    );
    println!(
        "Test 47 left of 53: {}",
        task.is_valid(53, Direction::Left, 47)
    );

    println!("Should be false");
    println!(
        "Test 53 left of 47: {}",
        task.is_valid(47, Direction::Left, 53)
    );
    println!(
        "Test 47 right of 53: {}",
        task.is_valid(53, Direction::Right, 47)
    );

    println!("Result part one: {}", task.solve_part_one());
    println!("Result part two: {}", task.solve_part_two());
}
