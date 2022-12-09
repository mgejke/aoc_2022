use std::collections::HashMap;

use itertools::Itertools;

fn get_stacks(stacks_str: &str) -> HashMap<u32, Vec<char>> {
    let mut stacks: HashMap<u32, Vec<char>> = HashMap::new();
    for line in stacks_str.lines().rev() {
        for (c, letter) in line.chars().skip(1).step_by(4).enumerate() {
            if letter.is_uppercase() {
                stacks.entry((c + 1) as u32).or_default().push(letter);
            }
        }
    }
    stacks
}

pub fn get_instructions(instructions_str: &str) -> Vec<(u32, u32, u32)> {
    instructions_str
        .trim()
        .lines()
        .map(|line| {
            let (_, count, _, from, _, to) = line.split_whitespace().collect_tuple().unwrap();
            (
                count.parse().unwrap(),
                from.parse().unwrap(),
                to.parse().unwrap(),
            )
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks_str, instructions_str) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks = get_stacks(stacks_str);
    for (count, from, to) in get_instructions(instructions_str) {
        for _i in 0..count {
            let c = stacks.get_mut(&from).unwrap().pop().unwrap();
            stacks.get_mut(&to).unwrap().push(c);
        }
    }
    Some(
        (1..stacks.len() + 1)
            .map(|v| stacks.get_mut(&(v as u32)).unwrap().pop().unwrap())
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks_str, instructions_str) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks = get_stacks(stacks_str);
    for (count, from, to) in get_instructions(instructions_str) {
        let stack = stacks.get_mut(&from).unwrap();
        let mut temp = stack.split_off(stack.len() - count as usize);
        stacks.get_mut(&to).unwrap().append(&mut temp);
    }
    Some(
        (1..stacks.len() + 1)
            .map(|v| stacks.get_mut(&(v as u32)).unwrap().pop().unwrap())
            .collect(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
