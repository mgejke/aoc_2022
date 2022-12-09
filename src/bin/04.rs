use std::collections::HashSet;

use itertools::Itertools;

pub fn to_hashsets(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let (s1, s2) = line.split(',').collect_tuple().unwrap();
    (to_range_hashset(s1), to_range_hashset(s2))
}

pub fn to_range_hashset(values: &str) -> HashSet<u32> {
    let (v1, v2) = values.split('-').collect_tuple().unwrap();
    HashSet::from_iter(v1.parse::<u32>().unwrap()..v2.parse::<u32>().unwrap() + 1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let count: usize = input
        .trim()
        .split('\n')
        .map(to_hashsets)
        .filter(|(r1, r2)| r1.is_subset(r2) || r2.is_subset(r1))
        .count();
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count: usize = input
        .trim()
        .split('\n')
        .map(to_hashsets)
        .filter(|(r1, r2)| r1.intersection(r2).count() > 0)
        .count();
    Some(count as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
