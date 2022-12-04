use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../input/day04.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(s: &str) -> i32 {
    overlap_pairs(parse(s))
}

fn overlap_pairs(pairs: Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>) -> i32 {
    pairs
        .iter()
        .filter_map(|p| {
            if (p.0.contains(p.1.start()) || p.0.contains(p.1.end()))
                || (p.1.contains(p.0.start()) || p.1.contains(p.0.end()))
            {
                Some(1)
            } else {
                None
            }
        })
        .sum()
}

fn parse(s: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    s.lines()
        .map(|line| {
            let (l, r) = line.split_once(',').expect("comma delineated lines");

            fn to_range(s: &str) -> RangeInclusive<i32> {
                let (l_str, r_str) = s.split_once('-').expect("dash delineated pairs");
                let (l, r) = (l_str.parse().expect("num"), r_str.parse().expect("num"));

                l..=r
            }

            (to_range(l), to_range(r))
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const WEB_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    const PARSED_WEB_INPUT: [(RangeInclusive<i32>, RangeInclusive<i32>); 6] = [
        ((2..=4), (6..=8)),
        ((2..=3), (4..=5)),
        ((5..=7), (7..=9)),
        ((2..=8), (3..=7)),
        ((6..=6), (4..=6)),
        ((2..=6), (4..=8)),
    ];

    #[test]
    fn web_parse() {
        assert_eq!(PARSED_WEB_INPUT.to_vec(), parse(WEB_INPUT));
    }

    #[test]
    fn web_test() {
        assert_eq!(4, overlap_pairs(PARSED_WEB_INPUT.to_vec()))
    }

    #[test]
    fn test() {
        assert_eq!(770, run(INPUT))
    }
}
