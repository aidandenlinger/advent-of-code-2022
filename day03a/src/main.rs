use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(s: &str) -> i32 {
    parse(s).iter().map(find_error).map(priority).sum()
}

fn parse(s: &str) -> Vec<Knapsack> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

fn find_error(k: &Knapsack) -> char {
    let l_keys: HashSet<&char> = k.l.0.keys().collect();
    let r_keys: HashSet<&char> = k.r.0.keys().collect();

    **l_keys.intersection(&r_keys).next().unwrap()
}

fn priority(c: char) -> i32 {
    assert!(c.is_ascii_alphabetic());

    if c.is_ascii_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    }
}

struct Compartment(HashMap<char, i32>);

struct Knapsack {
    l: Compartment,
    r: Compartment,
}

impl FromStr for Knapsack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l_str, r_str) = s.split_at(s.len() / 2);
        let (mut l, mut r) = (HashMap::new(), HashMap::new());

        for c in l_str.chars() {
            *l.entry(c).or_insert(0) += 1;
        }

        for c in r_str.chars() {
            *r.entry(c).or_insert(0) += 1;
        }

        Ok(Knapsack {
            l: Compartment(l),
            r: Compartment(r),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_error() {
        assert_eq!(
            'p',
            find_error(&WEB_EXAMPLE.lines().next().unwrap().parse().unwrap())
        );
        assert_eq!(
            'L',
            find_error(&WEB_EXAMPLE.lines().nth(1).unwrap().parse().unwrap())
        );
        assert_eq!(
            'P',
            find_error(&WEB_EXAMPLE.lines().nth(2).unwrap().parse().unwrap())
        );
        assert_eq!(
            'v',
            find_error(&WEB_EXAMPLE.lines().nth(3).unwrap().parse().unwrap())
        );
        assert_eq!(
            't',
            find_error(&WEB_EXAMPLE.lines().nth(4).unwrap().parse().unwrap())
        );
        assert_eq!(
            's',
            find_error(&WEB_EXAMPLE.lines().nth(5).unwrap().parse().unwrap())
        );
    }

    #[test]
    fn test_priority() {
        assert_eq!(16, priority('p'));
        assert_eq!(38, priority('L'));
        assert_eq!(42, priority('P'));
        assert_eq!(22, priority('v'));
        assert_eq!(20, priority('t'));
        assert_eq!(19, priority('s'));
    }

    #[test]
    fn test_web() {
        assert_eq!(157, run(WEB_EXAMPLE))
    }

    #[test]
    fn test() {
        assert_eq!(7597, run(INPUT))
    }
}
