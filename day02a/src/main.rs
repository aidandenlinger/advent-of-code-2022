const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", calc_score(parse(INPUT)));
}

fn parse(s: &str) -> Vec<(Move, Move)> {
    s.lines().map(parse_move_pair).collect()
}

fn parse_move_pair(s: &str) -> (Move, Move) {
    let (a, b) = s.split_once(' ').unwrap();
    (
        Move::new(a.chars().next().unwrap()),
        Move::new(b.chars().next().unwrap()),
    )
}

fn calc_score(m: Vec<(Move, Move)>) -> i32 {
    m.iter()
        .map(|(opponent, me)| me.points() + me.beats(opponent).points())
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
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
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

    // Determines if this move beats the parameter move.
    fn beats(&self, o: &Move) -> GameOutcome {
        if *self == *o {
            GameOutcome::Draw
        } else {
            match self {
                Move::Rock if matches!(o, Move::Scissors) => GameOutcome::Win,
                Move::Paper if matches!(o, Move::Rock) => GameOutcome::Win,
                Move::Scissors if matches!(o, Move::Paper) => GameOutcome::Win,
                _ => GameOutcome::Lose,
            }
        }
    }
}

impl GameOutcome {
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

    const PARSED_WEB_EXAMPLE: [(Move, Move); 3] = [
        (Move::Rock, Move::Paper),
        (Move::Paper, Move::Rock),
        (Move::Scissors, Move::Scissors),
    ];

    #[test]
    fn parse_web() {
        assert_eq!(PARSED_WEB_EXAMPLE.to_vec(), parse(WEB_EXAMPLE));
    }

    #[test]
    fn test_web() {
        assert_eq!(15, calc_score(PARSED_WEB_EXAMPLE.to_vec()));
    }

    #[test]
    fn answer() {
        assert_eq!(15691, calc_score(parse(INPUT)));
    }
}
