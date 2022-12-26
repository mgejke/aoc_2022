use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use advent_of_code::helpers::DIRECTIONS;
use rusttype::Point;

type P = Point<i32>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct MapPos {
    cost: u32,
    position: P,
    height: i32,
}

impl Ord for MapPos {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for MapPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_map(
    start_position: P,
    map: &HashMap<P, i32>,
    check_height: impl Fn(i32, i32) -> bool,
    exit_condition: impl Fn(&P) -> bool,
) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    heap.push(MapPos {
        cost: 0,
        position: start_position,
        height: *map.get(&start_position).unwrap(),
    });
    let mut visited: HashSet<P> = HashSet::new();

    while let Some(MapPos {
        cost,
        position,
        height,
    }) = heap.pop()
    {
        if exit_condition(&position) {
            return Some(cost);
        };

        for direction in &DIRECTIONS {
            let next_position = position + *direction;
            if let Some(next_height) = map.get(&next_position) {
                if visited.contains(&next_position) {
                    continue;
                }
                if check_height(*next_height, height) {
                    heap.push(MapPos {
                        cost: cost + 1,
                        position: next_position,
                        height: *next_height,
                    });
                    visited.insert(next_position);
                }
            }
        }
    }
    // }
    None
}

fn get_map(input: &str, start_position: &mut P, end_position: &mut P) -> HashMap<P, i32> {
    let mut map: HashMap<P, i32> = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    *start_position = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    map.entry(*start_position).or_insert(0);
                }
                'E' => {
                    *end_position = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    map.entry(*end_position).or_insert('z' as i32 - 'a' as i32);
                }
                c => {
                    let p = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    map.entry(p).or_insert(c as i32 - 'a' as i32);
                }
            }
        }
    }
    map
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start_position: P = Point { x: 0, y: 0 };
    let mut end_position: P = Point { x: 0, y: 0 };
    let map = get_map(input, &mut start_position, &mut end_position);
    solve_map(
        start_position,
        &map,
        |next, current| next - current <= 1,
        |p| *p == end_position,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start_position: P = Point { x: 0, y: 0 };
    let mut end_position: P = Point { x: 0, y: 0 };
    let map = get_map(input, &mut start_position, &mut end_position);
    solve_map(
        end_position,
        &map,
        |next, current| current - next <= 1,
        |p| *map.get(p).unwrap() == 0,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
