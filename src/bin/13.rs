use itertools::{all, Itertools};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Signal {
    Value(u32),
    List(Vec<Signal>),
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Signal::Value(l0), Signal::Value(r0)) => l0.cmp(r0),
            (Signal::Value(_), Signal::List(_)) => Signal::List(vec![self.clone()]).cmp(other),
            (Signal::List(_), Signal::Value(_)) => self.cmp(&Signal::List(vec![other.clone()])),
            (Signal::List(l0), Signal::List(r0)) => {
                for pair in l0.iter().zip_longest(r0) {
                    match pair {
                        itertools::EitherOrBoth::Both(_, _) => {
                            if l0 != r0 {
                                return l0.cmp(r0);
                            }
                        }
                        itertools::EitherOrBoth::Left(_) => return Ordering::Greater,
                        itertools::EitherOrBoth::Right(_) => return Ordering::Less,
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Signal::Value(l0), Signal::Value(r0)) => l0 == r0,
            (Signal::Value(_), Signal::List(_)) => Signal::List(vec![self.clone()]).eq(other),
            (Signal::List(_), Signal::Value(_)) => self.eq(&Signal::List(vec![other.clone()])),
            (Signal::List(l0), Signal::List(r0)) => {
                if l0.len() != r0.len() {
                    false
                } else {
                    all(l0.iter().zip(r0), |(l0, r0)| l0.eq(r0))
                }
            }
        }
    }
}

impl Eq for Signal {
    fn assert_receiver_is_total_eq(&self) {}
}

fn parse_signal(input: &mut std::iter::Peekable<std::str::Chars>) -> Signal {
    let mut signal: Vec<Signal> = Vec::new();
    if let Some(_o_bracket @ '[') = input.next() {
        'outer: loop {
            let mut value = "".to_string();
            'inner: loop {
                match input.peek() {
                    Some(_n @ ',') => {
                        input.next();
                        if let Ok(c) = value.parse() {
                            signal.push(Signal::Value(c));
                        }
                        break 'inner;
                    }
                    Some(_n @ ']') => {
                        input.next();
                        if let Ok(c) = value.parse() {
                            signal.push(Signal::Value(c));
                        }
                        break 'outer;
                    }
                    Some(_n @ '[') => {
                        signal.push(parse_signal(input));
                    }
                    Some(_c) => {
                        let c = input.next().unwrap();
                        value.push(c);
                    }
                    None => panic!("Huh?"),
                }
            }
        }
    } else {
        panic!("No start bracket!?")
    }
    Signal::List(signal)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for (i, pairs) in input.trim().split("\n\n").enumerate() {
        let (n1, n2) = pairs.split_once('\n').unwrap();
        let s1: Signal = parse_signal(&mut n1.chars().peekable());
        let s2: Signal = parse_signal(&mut n2.chars().peekable());
        if s2 > s1 {
            numbers.push(i as u32 + 1);
        }
    }
    Some(numbers.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let s1 = parse_signal(&mut "[[2]]".chars().peekable());
    let s2 = parse_signal(&mut "[[6]]".chars().peekable());

    let signals = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_signal(&mut line.chars().peekable()))
        .chain([s1.clone(), s2.clone()])
        .sorted()
        .collect_vec();

    let i1 = signals.binary_search(&s1).unwrap() + 1;
    let i2 = signals.binary_search(&s2).unwrap() + 1;

    Some(i1 * i2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_equal() {
        let n1 = "[1,1,3,1,1]";
        let n2 = "[1,1,3,1,1]";
        let s1 = parse_signal(&mut n1.chars().peekable());
        let s2 = parse_signal(&mut n2.chars().peekable());

        assert_eq!(s1, s2);
    }

    #[test]
    fn test_not_equal() {
        let n1 = "[1,1,3,1,1]";
        let n2 = "[1,1,5,1,1]";
        let s1 = parse_signal(&mut n1.chars().peekable());
        let s2 = parse_signal(&mut n2.chars().peekable());

        assert_ne!(s1, s2);
    }
    #[test]
    fn test_list_single_to_list() {
        let n1 = "[[1]]";
        let n2 = "[1]";
        let s1 = parse_signal(&mut n1.chars().peekable());
        let s2 = parse_signal(&mut n2.chars().peekable());

        assert_eq!(s1, s2);
    }
    #[test]
    fn test_lt() {
        let n1 = "[]";
        let n2 = "[3]";
        let s1 = parse_signal(&mut n1.chars().peekable());
        let s2 = parse_signal(&mut n2.chars().peekable());

        assert!(s2 > s1);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
