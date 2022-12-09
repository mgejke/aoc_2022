use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let combos = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2),
        ("C Z", 3 + 3),
    ]);
    let rows = input.trim().split('\n');
    let score: u32 = rows.into_iter().map(|x| combos[x]).sum();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.trim().split('\n').collect();
    let combos = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2),
        ("C Z", 3 + 3),
    ]);

    let lose_map = HashMap::from([('A', 'Z'), ('B', 'X'), ('C', 'Y')]);
    let win_map = HashMap::from([('A', 'Y'), ('B', 'Z'), ('C', 'X')]);
    let draw_map = HashMap::from([('A', 'X'), ('B', 'Y'), ('C', 'Z')]);

    let mut new_combos: Vec<String> = Vec::new();
    for row in rows {
        let first = row.chars().next().unwrap();
        let last = row.chars().last().unwrap();
        let choice = match last {
            'X' => lose_map[&first],
            'Y' => draw_map[&first],
            'Z' => win_map[&first],
            _ => panic!(),
        };
        let result = format!("{first} {choice}");
        new_combos.push(result);
    }
    let score: u32 = new_combos.into_iter().map(|x| combos[x.as_str()]).sum();
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
