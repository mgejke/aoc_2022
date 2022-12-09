use itertools::Itertools;

fn find_unique_sequence(input: &str, window_size: usize) -> Option<usize> {
    for (i, window) in input.chars().collect_vec().windows(window_size).enumerate() {
        if window.iter().unique().count() == window_size {
            return Some(i + window_size);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    find_unique_sequence(input.trim(), 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    find_unique_sequence(input.trim(), 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
