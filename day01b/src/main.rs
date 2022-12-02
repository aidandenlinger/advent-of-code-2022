fn main() {
    println!("{}", find_top_three(parse(include_str!("../input.txt"))));
}

// Given input, sums up each of the inner arrays and returns the sum of the
// largest three elements.
fn find_top_three(i: Vec<Vec<i32>>) -> i32 {
    let mut sorted: Vec<i32> = i.iter().map(|j| j.iter().sum()).collect();
    sorted.sort_unstable();
    return sorted.get(sorted.len() - 3..).unwrap().iter().sum();
}

// Given input string, split it into groups based off of newlines between
// them and parse those into numbers.
fn parse(s: &str) -> Vec<Vec<i32>> {
    s.split("\n\n")
        .map(|group| group.lines().map(|l| l.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn parse_web() {
        assert_eq!(
            vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10000]
            ],
            parse(WEB_EXAMPLE)
        )
    }

    #[test]
    fn answer_web() {
        assert_eq!(45000, find_top_three(parse(WEB_EXAMPLE)))
    }
}
