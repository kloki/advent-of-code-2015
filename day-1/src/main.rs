use std::fs;
fn main() {
    let input_file = "./input/input.txt".to_owned();
    let contents = fs::read_to_string(input_file).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
    let answer = run_2(contents.clone());
    println!("{}", answer);
}

fn run_1(contents: String) -> isize {
    let up = contents
        .chars()
        .filter(|x| *x == '(')
        .collect::<Vec<_>>()
        .len() as isize;

    let down = contents
        .chars()
        .filter(|x| *x == ')')
        .collect::<Vec<_>>()
        .len() as isize;
    return up - down;
}
fn run_2(contents: String) -> usize {
    let mut current_level = 0;
    for (i, c) in contents.chars().enumerate() {
        match c {
            ')' => current_level -= 1,
            '(' => current_level += 1,
            _ => {}
        }
        if current_level < 0 {
            return i + 1;
        }
    }
    0
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day1_1() {
        assert_eq!(run_1("(())".to_string()), 0);
        assert_eq!(run_1("()()".to_string()), 0);
        assert_eq!(run_1("(((".to_string()), 3);
        assert_eq!(run_1("(()(()(".to_string()), 3);
        assert_eq!(run_1("())".to_string()), -1);
        assert_eq!(run_1("))(".to_string()), -1);
        assert_eq!(run_1(")))".to_string()), -3);
        assert_eq!(run_1(")())())".to_string()), -3);
    }
    #[test]
    fn test_day1_2() {
        assert_eq!(run_2(")".to_string()), 1);
        assert_eq!(run_2("()())".to_string()), 5);
    }
}
