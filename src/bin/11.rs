use itertools::Itertools;
use std::{collections::VecDeque, str::FromStr};

#[derive(Clone, Copy)]
enum Operation {
    AddN(u64),
    Add(),
    MulN(u64),
    Mul(),
}

trait Op {
    fn operate(self, v: u64) -> u64;
}

impl Op for Operation {
    fn operate(self, v: u64) -> u64 {
        match self {
            Operation::AddN(n) => n + v,
            Operation::Add() => v + v,
            Operation::MulN(n) => n * v,
            Operation::Mul() => v * v,
        }
    }
}

struct Monkey {
    items: VecDeque<u64>,
    divisor: u64,
    on_true: usize,
    on_false: usize,
    operation: Operation,
    item_counter: u64,
}

fn get_last_val<T: FromStr>(str_: &str) -> Option<T> {
    let (_, divisor_str) = str_.split_once(':').unwrap();
    match divisor_str.split_whitespace().last().unwrap().parse::<T>() {
        Ok(v) => Some(v),
        _ => None,
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let mut itr = input.lines().skip(1);
    let (_, item_str) = itr.next().unwrap().split_once(':').unwrap();
    let items = item_str
        .split(',')
        .map(|v| v.trim().parse::<u64>().unwrap())
        .collect::<VecDeque<u64>>();

    let (_, operation_str) = itr.next().unwrap().split_once(':').unwrap();
    let (_, expression) = operation_str.split_once('=').unwrap();
    let (n1, op, n2) = expression.split_whitespace().collect_tuple().unwrap();
    let operation = match op {
        "+" => {
            if n1 == n2 {
                Operation::Add()
            } else {
                Operation::AddN(n2.parse::<u64>().unwrap())
            }
        }
        "*" => {
            if n1 == n2 {
                Operation::Mul()
            } else {
                Operation::MulN(n2.parse::<u64>().unwrap())
            }
        }
        _ => panic!("unsupported operation"),
    };

    let divisor: u64 = get_last_val(itr.next().unwrap()).unwrap();
    let on_true: usize = get_last_val(itr.next().unwrap()).unwrap();
    let on_false: usize = get_last_val(itr.next().unwrap()).unwrap();

    Monkey {
        items,
        divisor,
        on_true,
        on_false,
        operation,
        item_counter: 0,
    }
}

fn monkey_stuff(mut monkeys: Vec<Monkey>, rounds: u32, do_div: bool) -> u64 {
    let all_primes_product: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(worry_level) = monkeys[i].items.pop_front() {
                let mut new_level: u64 = monkeys[i].operation.operate(worry_level);
                if do_div {
                    new_level = (new_level as f64 / 3.0).floor() as u64
                } else {
                    new_level %= all_primes_product;
                }
                let rest = new_level % monkeys[i].divisor;
                let new_monkey = if rest == 0 {
                    monkeys[i].on_true
                } else {
                    monkeys[i].on_false
                };

                monkeys[new_monkey].items.push_back(new_level);
                monkeys[i].item_counter += 1;
            }
        }
    }
    monkeys
        .iter()
        .map(|m| m.item_counter)
        .sorted_by(|a, b| a.cmp(b))
        .rev()
        .take(2)
        .product()
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = input.trim().split("\n\n").map(parse_monkey).collect_vec();
    let s = monkey_stuff(monkeys, 20, true);
    Some(s)
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkeys = input.trim().split("\n\n").map(parse_monkey).collect_vec();
    let s = monkey_stuff(monkeys, 10000, false);
    Some(s)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
