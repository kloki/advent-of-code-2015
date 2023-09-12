use std::{
    collections::HashSet,
    fs,
};
fn main() {
    let input_file = "./input/input.txt".to_owned();
    let contents = fs::read_to_string(input_file).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
}

enum Operation {
    On,
    Off,
    Toggle,
}

fn parse_action(input: &str) -> (Operation, usize, usize, usize, usize) {
    let parts: Vec<_> = input.split(" ").collect();
    let operation = match parts[1] {
        "on" => Operation::On,
        "off" => Operation::Off,
        _ => Operation::Toggle,
    };

    let (x_index, y_index) = match operation {
        Operation::Toggle => (1, 3),
        _ => (2, 4),
    };
    let xparts: Vec<_> = parts[x_index].split(",").collect();
    let yparts: Vec<_> = parts[y_index].split(",").collect();

    (
        operation,
        xparts[0].parse().unwrap(),
        xparts[1].parse().unwrap(),
        yparts[0].parse().unwrap(),
        yparts[1].parse().unwrap(),
    )
}

fn run_1(contents: String) -> usize {
    let actions = contents
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| parse_action(x));
    let mut lights: HashSet<(usize, usize)> = HashSet::new();

    for (action, x1, y1, x2, y2) in actions {
        for y in y1..=y2 {
            for x in x1..=x2 {
                match action {
                    Operation::On => {
                        lights.insert((x, y));
                    }
                    Operation::Off => {
                        lights.remove(&(x, y));
                    }
                    Operation::Toggle => {
                        if !lights.insert((x, y)) {
                            lights.remove(&(x, y));
                        };
                    }
                };
            }
        }
    }

    lights.len()
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day1_1_on() {
        assert_eq!(run_1("turn on 0,0 through 2,2".to_string()), 9);
    }
    #[test]
    fn test_day1_1_off() {
        assert_eq!(run_1("turn off 0,0 through 2,2".to_string()), 0);
    }
    #[test]
    fn test_day1_1_toggle() {
        assert_eq!(run_1("toggle 0,0 through 2,2".to_string()), 9);
    }
}
