use std::fs;
fn main() {
    let input_file = "./input/input.txt".to_owned();
    let contents = fs::read_to_string(input_file).expect("Can't read file");
    let boxes = contents
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| Box::new(x));
    println!("{}", boxes.clone().map(|x| x.size()).sum::<usize>());
    println!("{}", boxes.map(|x| x.ribbon()).sum::<usize>());
}

struct Box {
    x: usize,
    y: usize,
    z: usize,
}

impl Box {
    fn new(input: &str) -> Self {
        let parts = input.split("x").collect::<Vec<_>>();
        Box {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        }
    }
    fn size(&self) -> usize {
        let scores = vec![self.x * self.y, self.y * self.z, self.x * self.z];
        2 * scores.iter().sum::<usize>() + scores.iter().min().unwrap()
    }
    fn ribbon(&self) -> usize {
        let mut sizes = vec![self.x, self.y, self.z];
        sizes.sort();
        2 * (sizes[0] + sizes[1]) + sizes.iter().product::<usize>()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day2_1() {
        assert_eq!(Box::new("2x3x4").size(), 58);
        assert_eq!(Box::new("1x1x10").size(), 43);
    }
    #[test]
    fn test_day2_2() {
        assert_eq!(Box::new("2x3x4").ribbon(), 34);
        assert_eq!(Box::new("1x1x10").ribbon(), 14);
    }
}
