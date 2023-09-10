use std::{
    collections::HashSet,
    fs,
};
fn main() {
    let input_file = "./input/input.txt".to_owned();
    let contents = fs::read_to_string(input_file).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
    let answer = run_2(contents.clone());
    println!("{}", answer);
}

struct Journey {
    visited: HashSet<(isize, isize)>,
    location: (isize, isize),
    location_robot: (isize, isize),
}

impl Journey {
    fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        Journey {
            visited,
            location: (0, 0),
            location_robot: (0, 0),
        }
    }

    fn len(&self) -> usize {
        self.visited.len()
    }

    fn santa_run(&mut self, input: String) {
        for c in input.chars() {
            match c {
                '<' => self.location.0 -= 1,
                '>' => self.location.0 += 1,
                '^' => self.location.1 -= 1,
                'v' => self.location.1 += 1,
                _ => {}
            }
            self.visited.insert(self.location);
        }
    }
    fn santa_robot_run(&mut self, input: String) {
        let mut toggle = true;
        for c in input.chars() {
            match (c, toggle) {
                ('<', true) => self.location.0 -= 1,
                ('>', true) => self.location.0 += 1,
                ('^', true) => self.location.1 -= 1,
                ('v', true) => self.location.1 += 1,
                ('<', false) => self.location_robot.0 -= 1,
                ('>', false) => self.location_robot.0 += 1,
                ('^', false) => self.location_robot.1 -= 1,
                ('v', false) => self.location_robot.1 += 1,
                _ => {}
            }
            if toggle {
                self.visited.insert(self.location);
            } else {
                self.visited.insert(self.location_robot);
            }
            toggle ^= true;
        }
    }
}

fn run_1(contents: String) -> usize {
    let mut journey = Journey::new();
    journey.santa_run(contents);
    journey.len()
}
fn run_2(contents: String) -> usize {
    let mut journey = Journey::new();
    journey.santa_robot_run(contents);
    journey.len()
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day3_1() {
        let mut journey = Journey::new();
        journey.santa_run(">".to_string());
        assert_eq!(journey.len(), 2);
        let mut journey = Journey::new();
        journey.santa_run("^>v<".to_string());
        assert_eq!(journey.len(), 4);
        let mut journey = Journey::new();
        journey.santa_run("^v^v^v^v^v".to_string());
        assert_eq!(journey.len(), 2);
    }
    #[test]
    fn test_day3_2() {
        let mut journey = Journey::new();
        journey.santa_robot_run("^v".to_string());
        assert_eq!(journey.len(), 3);
        let mut journey = Journey::new();
        journey.santa_robot_run("^>v<".to_string());
        assert_eq!(journey.len(), 3);
        let mut journey = Journey::new();
        journey.santa_robot_run("^v^v^v^v^v".to_string());
        assert_eq!(journey.len(), 11);
    }
}
