fn main() {
    let answer = find_hash("bgvyzdsv".to_string(), "00000");
    println!("{}", answer);
    let answer = find_hash("bgvyzdsv".to_string(), "000000");
    println!("{}", answer);
}

fn find_hash(contents: String, starts_width: &str) -> usize {
    let mut current = 60903;
    loop {
        let digest = md5::compute(format!("{}{}", contents, current));

        if format!("{:?}", digest).starts_with(starts_width) {
            break;
        }
        current += 1
    }
    current
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day1_1() {
        assert_eq!(find_hash("abcdef".to_string(), "00000"), 609043);
        assert_eq!(find_hash("pqrstuv".to_string(), "00000"), 1048970);
    }
}
