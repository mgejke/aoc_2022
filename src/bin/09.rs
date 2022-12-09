use std::collections::HashMap;

use itertools::Itertools;
use rusttype::{Point, Vector};

pub fn part_one(input: &str) -> Option<u32> {
    let directions: HashMap<char, Vector<i32>> = HashMap::from_iter([
        ('L', Vector { x: -1, y: 0 }),
        ('R', Vector { x: 1, y: 0 }),
        ('U', Vector { x: 0, y: -1 }),
        ('D', Vector { x: 0, y: 1 }),
    ]);

    let instructions = input.trim().lines().map(|line| {
        let (d, v) = line.split_whitespace().collect_tuple().unwrap();
        (d.chars().next().unwrap(), v.parse::<i32>().unwrap())
    });

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    let mut visited: HashMap<Point<i32>, bool> = HashMap::from_iter([(head, true)]);
    for (d, v) in instructions {
        let dir = directions.get(&d).unwrap();
        for _ in 0..v {
            head = head + *dir;
            let mut mov_dir = head - tail;
            if mov_dir.x.abs() > 1 || mov_dir.y.abs() > 1 {
                mov_dir.x = if mov_dir.x != 0 {
                    mov_dir.x / mov_dir.x.abs()
                } else {
                    0
                };
                mov_dir.y = if mov_dir.y != 0 {
                    mov_dir.y / mov_dir.y.abs()
                } else {
                    0
                };
                tail = tail + mov_dir;
                visited.entry(tail).or_insert(true);
            }
        }
    }

    let sum = visited.len();

    Some(sum as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
