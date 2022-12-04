use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("../../input/day03.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(s: &str) -> i32 {
    parse(s).iter().map(|g| find_error(g)).map(priority).sum()
}

fn parse(s: &str) -> Vec<Vec<Knapsack>> {
    s.lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|x| x.to_vec())
        .collect()
}

fn find_error(k: &[Knapsack]) -> char {
    assert!(k.len() == 3);
    *k[0]
        .0
        .intersection(&k[1].0)
        .copied()
        .collect::<HashSet<_>>()
        .intersection(&k[2].0)
        .next()
        .unwrap()
}

fn priority(c: char) -> i32 {
    assert!(c.is_ascii_alphabetic());

    if c.is_ascii_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    }
}

#[derive(Clone)]
struct Knapsack(HashSet<char>);

impl FromStr for Knapsack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Knapsack(s.chars().collect()))
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
    fn test_web() {
        assert_eq!(70, run(WEB_EXAMPLE))
    }

    #[test]
    fn test() {
        assert_eq!(2607, run(INPUT))
    }
}
