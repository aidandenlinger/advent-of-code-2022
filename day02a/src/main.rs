/*
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be
closest to the snack storage, a giant Rock Paper Scissors tournament is already
in progress.

Rock Paper Scissors is a game between two players. Each game contains many
rounds; in each round, the players each simultaneously choose one of Rock,
Paper, or Scissors using a hand shape. Then, a winner for that round is
selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy
guide (your puzzle input) that they say will be sure to help you win. "The first
column is what your opponent is going to play: A for Rock, B for Paper, and C
for Scissors. The second column--" Suddenly, the Elf is called away to help with
someone's tent.

The second column, you reason, must be what you should play in response: X for
Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious,
so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your
total score is the sum of your scores for each round. The score for a single
round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3
for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if
the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you
should calculate the score you would get if you were to follow the strategy
guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

- In the first round, your opponent will choose Rock (A), and you should choose
Paper (Y). This ends in a win for you with a score of 8 (2 because you chose
Paper + 6 because you won).
- In the second round, your opponent will choose Paper (B), and you should
choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
- The third round is a draw with both players choosing Scissors, giving you a
score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total
score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your
strategy guide?
*/

fn main() {
    println!("{}", calc_score(parse(include_str!("../input.txt"))));
}

fn parse(s: &str) -> Vec<(Move, Move)> {
    s.lines().map(parse_move_pair).collect()
}

fn parse_move_pair(s: &str) -> (Move, Move) {
    assert_eq!(3, s.len());
    (
        Move::new(s.chars().next().unwrap()),
        Move::new(s.chars().nth(2).unwrap()),
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
}
