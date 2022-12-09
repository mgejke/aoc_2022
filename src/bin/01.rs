use itertools::sorted;

pub fn get_sum(input: &str, item_count: usize) -> u32 {
    let all = input.trim().split("\n\n").into_iter().map(|elf| {
        elf.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .sum::<u32>()
    });

    sorted(all).rev().take(item_count).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_sum(input, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_sum(input, 3))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
