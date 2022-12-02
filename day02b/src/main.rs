fn main() {
    println!("{}", calc_score(parse(include_str!("../input.txt"))));
}

fn parse(s: &str) -> Vec<(Move, GameOutcome)> {
    s.lines().map(parse_move_pair).collect()
}

fn parse_move_pair(s: &str) -> (Move, GameOutcome) {
    assert_eq!(3, s.len());
    (
        Move::new(s.chars().next().unwrap()),
        GameOutcome::new(s.chars().nth(2).unwrap()),
    )
}

fn my_move(opp: &Move, end: &GameOutcome) -> Move {
    match end {
        GameOutcome::Draw => opp.clone(),
        GameOutcome::Lose => match opp {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        GameOutcome::Win => match opp {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
    }
}

fn calc_score(m: Vec<(Move, GameOutcome)>) -> i32 {
    m.iter()
        .map(|(opp, end)| my_move(opp, end).points() + end.points())
        .sum()
}

#[derive(Debug, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, PartialEq)]
enum GameOutcome {
    Win,
    Lose,
    Draw,
}

impl Move {
    fn new(c: char) -> Self {
        match c {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn points(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl GameOutcome {
    fn new(c: char) -> Self {
        match c {
            'X' => GameOutcome::Lose,
            'Y' => GameOutcome::Draw,
            'Z' => GameOutcome::Win,
            _ => unreachable!(),
        }
    }

    fn points(&self) -> i32 {
        match self {
            GameOutcome::Win => 6,
            GameOutcome::Lose => 0,
            GameOutcome::Draw => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEB_EXAMPLE: &str = "\
A Y
B X
C Z";

    const PARSED_WEB_EXAMPLE: [(Move, GameOutcome); 3] = [
        (Move::Rock, GameOutcome::Draw),
        (Move::Paper, GameOutcome::Lose),
        (Move::Scissors, GameOutcome::Win),
    ];

    #[test]
    fn parse_web() {
        assert_eq!(PARSED_WEB_EXAMPLE.to_vec(), parse(WEB_EXAMPLE));
    }

    #[test]
    fn test_web() {
        assert_eq!(12, calc_score(PARSED_WEB_EXAMPLE.to_vec()));
    }
}
