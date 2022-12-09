use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn calc_score(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 'a' as u32 + 1
    } else {
        *c as u32 - 'A' as u32 + 27
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.split('\n') {
        let half = line.len() / 2;
        let begin: HashSet<char> = HashSet::from_iter(line.chars().take(half));
        let end: HashSet<char> = HashSet::from_iter(line.chars().rev().take(half));

        sum += begin.intersection(&end).map(calc_score).sum::<u32>();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for backpacks in &input.trim().split('\n').chunks(3) {
        let mut counts: HashMap<char, u32> = HashMap::new();
        for backpack in backpacks {
            let set: HashSet<char> = HashSet::from_iter(backpack.chars());
            for c in set {
                *counts.entry(c).or_insert(0) += 1;
            }
        }
        let (k, _v) = counts.iter().find_or_last(|(_k, v)| **v == 3).unwrap();
        sum += calc_score(k);
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
