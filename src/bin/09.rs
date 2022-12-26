use std::collections::HashMap;

use itertools::Itertools;
use rusttype::{Point, Vector};

fn limit_vector(mut mov_dir: Vector<i32>) -> Option<Vector<i32>> {
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
        return Some(mov_dir);
    }
    None
}

fn do_rope_physics(
    instructions: Vec<(char, i32)>,
    directions: HashMap<char, Vector<i32>>,
    rope_lenght: usize,
) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut rope: Vec<Point<i32>> = vec![Point { x: 0, y: 0 }; rope_lenght];
    let mut visited: HashMap<Point<i32>, bool> = HashMap::new();
    for (d, v) in instructions {
        let dir = directions.get(&d).unwrap();
        for _ in 0..v {
            head = head + *dir;
            let mut previous = head;
            for knot in rope.iter_mut() {
                let mov_dir = previous - *knot;
                if let Some(v) = limit_vector(mov_dir) {
                    *knot = *knot + v;
                }
                previous = *knot;
            }
            visited.entry(*rope.last().unwrap()).or_insert(true);
        }
    }
    visited.len()
}

fn parse_instructions(input: &str) -> Vec<(char, i32)> {
    let instructions = input
        .trim()
        .lines()
        .map(|line| {
            let (d, v) = line.split_whitespace().collect_tuple().unwrap();
            (d.chars().next().unwrap(), v.parse::<i32>().unwrap())
        })
        .collect_vec();
    instructions
}
fn get_directions() -> HashMap<char, Vector<i32>> {
    let directions: HashMap<char, Vector<i32>> = HashMap::from_iter([
        ('L', Vector { x: -1, y: 0 }),
        ('R', Vector { x: 1, y: 0 }),
        ('U', Vector { x: 0, y: -1 }),
        ('D', Vector { x: 0, y: 1 }),
    ]);
    directions
}

pub fn part_one(input: &str) -> Option<u32> {
    let directions = get_directions();
    let instructions = parse_instructions(input);
    let sum = do_rope_physics(instructions, directions, 1);
    Some(sum as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    let directions = get_directions();
    let instructions = parse_instructions(input);
    let sum = do_rope_physics(instructions, directions, 9);
    Some(sum as u32)
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
        assert_eq!(part_two(&input), Some(1));
    }
}
