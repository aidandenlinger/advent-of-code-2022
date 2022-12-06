use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day06.txt");

fn main() {
    println!("{}", run(INPUT).unwrap());
}

fn run(s: &str) -> Option<usize> {
    for (idx, chars) in s.chars().collect::<Vec<_>>().windows(4).enumerate() {
        let set: HashSet<char> = chars.iter().copied().collect();
        if set.len() == 4 {
            return Some(idx + 4); // inc by 4
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEB_EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_web() {
        assert_eq!(7, run(WEB_EXAMPLE).unwrap())
    }

    #[test]
    fn test() {
        assert_eq!(0, run(INPUT).unwrap())
    }
}
