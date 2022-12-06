use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("../../input/day05.txt");

fn main() {
    println!("{:?}", run(INPUT));
}

fn parse(s: &str) -> (HashMap<usize, Vec<char>>, Vec<Move>) {
    let (crates_s, instr_s) = s
        .split_once("\n\n")
        .expect("empty line between crates and instrs");

    // Step 1: parse crates
    let mut iter = crates_s.lines();
    iter.next_back(); // consume last line, which is just numbers

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    for line in iter {
        for (idx, char) in line
            .chars()
            .skip(1) // skip first char
            .step_by(4) // skip by 4 to get crate letters
            .enumerate()
            .filter(|(_, char)| char.is_ascii_alphabetic())
        {
            // idx + 1 because we want to be 1-indexed and match
            // the input, so following Moves is super easy
            stacks.entry(idx + 1).or_default().insert(0, char);
        }
    }

    // Step 2: parse instructions

    let moves = instr_s
        .lines()
        .map(|l| {
            let mut words_iter = l.split_ascii_whitespace().skip(1).step_by(2);
            Move {
                amt: words_iter.next().unwrap().parse().expect("num"),
                from: words_iter.next().unwrap().parse().expect("num"),
                to: words_iter.next().unwrap().parse().expect("num"),
            }
        })
        .collect();

    (stacks, moves)
}

fn run(s: &str) -> String {
    let (mut crates, moves) = parse(s);

    for curr_move in moves {
        curr_move.apply(&mut crates);
    }

    let max_key = crates.keys().max().unwrap();

    let mut msg = String::new();
    for i in 1..=*max_key {
        msg.push(*crates.get(&i).unwrap().last().unwrap());
    }

    msg
}

#[derive(Debug, Clone, PartialEq)]
struct Move {
    amt: i32,
    from: usize,
    to: usize,
}

impl Move {
    fn apply(&self, stacks: &mut HashMap<usize, Vec<char>>) {
        let mut popped = VecDeque::new();
        for _ in 0..self.amt {
            popped.push_front(stacks.get_mut(&self.from).unwrap().pop().unwrap());
        }

        for c in popped {
            stacks.get_mut(&self.to).unwrap().push(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEB_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn parse_web() {
        assert_eq!(
            (
                HashMap::from([
                    (1, vec!['Z', 'N']),
                    (2, vec!['M', 'C', 'D']),
                    (3, vec!['P'])
                ]),
                vec![
                    Move {
                        amt: 1,
                        from: 2,
                        to: 1
                    },
                    Move {
                        amt: 3,
                        from: 1,
                        to: 3
                    },
                    Move {
                        amt: 2,
                        from: 2,
                        to: 1
                    },
                    Move {
                        amt: 1,
                        from: 1,
                        to: 2
                    }
                ]
            ),
            parse(WEB_INPUT)
        )
    }

    #[test]
    fn test_web() {
        assert_eq!("MCD", run(WEB_INPUT))
    }

    #[test]
    fn test() {
        assert_eq!("NLCDCLVMQ", run(INPUT))
    }
}
