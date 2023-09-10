use std::fs;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'u', 'o'];

fn main() {
    let input_file = "./input/input.txt".to_owned();
    let contents = fs::read_to_string(input_file).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
    let answer = run_2(contents.clone());
    println!("{}", answer);
}

fn run_1(contents: String) -> usize {
    contents.split("\n").filter(|x| is_nice(*x)).count()
}

fn run_2(contents: String) -> usize {
    contents.split("\n").filter(|x| is_nicer(*x)).count()
}
fn is_nice(input: &str) -> bool {
    if input.chars().filter(|x| VOWELS.contains(x)).count() < 3 {
        return false;
    }
    let mut doubles = false;
    let iter = input.chars().collect::<Vec<char>>();
    for wind in iter.windows(2) {
        match (wind[0], wind[1]) {
            (a, b) if a == b => doubles = true,
            ('a', 'b') => return false,
            ('c', 'd') => return false,
            ('p', 'q') => return false,
            ('x', 'y') => return false,
            _ => {}
        }
    }
    return doubles;
}
fn is_nicer(input: &str) -> bool {
    let iter = input.chars().collect::<Vec<char>>();
    if iter.len() == 0 {
        return false;
    }
    let mut pass = false;
    let mut pair: Vec<(char, char)> = Vec::new();
    for wind in iter.windows(3) {
        if wind[0] == wind[2] {
            pass = true;
        }

        if !(wind[0] == wind[1] && wind[1] == wind[2]) {
            pair.push((wind[0], wind[1]));
        }
    }
    //add the last one
    pair.push((iter[iter.len() - 2], iter[iter.len() - 1]));

    if !pass {
        return false;
    }

    let pre_len = pair.len();
    pair.sort();
    pair.dedup();
    pre_len != pair.len()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day5_1() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }
    #[test]
    fn test_day5_2() {
        assert_eq!(is_nicer("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nicer("xxyxx"), true);
        assert_eq!(is_nicer("uurcxstgmygtbstg"), false);
        assert_eq!(is_nicer("aaaxaa"), true);
        assert_eq!(is_nicer("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nicer("ieodomkazucvgmuy"), false);
        assert_eq!(is_nicer(""), false);
    }
}
