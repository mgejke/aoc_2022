use std::collections::HashMap;

use itertools::Itertools;
use rusttype::{Point, Vector};

fn parse_map(input: &str) -> (HashMap<Point<i32>, char>, Point<i32>, Point<i32>) {
    let mut map: HashMap<Point<i32>, char> = HashMap::new();
    let mut map_min = Point { x: i32::MAX, y: 0 };
    let mut map_max = Point { x: 0, y: 0 };
    for line in input.trim().lines() {
        let coords = line.split("->").map(|coords| {
            let (x, y) = coords.trim().split_once(',').unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            map_min.x = map_min.x.min(x);
            map_min.y = map_min.y.min(y);
            map_max.x = map_max.x.max(x);
            map_max.y = map_max.y.max(y);
            (x, y)
        });

        for ((x1, y1), (x2, y2)) in coords.tuple_windows() {
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    map.insert(Point { x: x1, y }, '#');
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    map.insert(Point { x, y: y1 }, '#');
                }
            }
        }
    }
    (map, map_min, map_max)
}

fn _print_map(map_min: Point<i32>, map_max: Point<i32>, map: &HashMap<Point<i32>, char>) {
    for y in map_min.y..=map_max.y {
        for x in map_min.x..=map_max.x {
            if let Some(c) = map.get(&Point { x, y }) {
                print!("{c}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, _map_min, map_max) = parse_map(input);

    let start_p: Point<i32> = Point { x: 500, y: 0 };
    let t = vec![
        Vector { x: 0, y: 1 },
        Vector { x: -1, y: 1 },
        Vector { x: 1, y: 1 },
    ];

    for counter in 0.. {
        let mut p: Point<i32> = start_p;

        loop {
            if let Some(v) = t.iter().find(|v| map.get(&(p + **v)).is_none()) {
                p = p + *v;
            } else {
                map.entry(p).or_insert('o');
                break;
            }

            if p.y > map_max.y {
                return Some(counter);
            }
        }
        // print_map(map_min, map_max, &map);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, _map_min, map_max) = parse_map(input);

    let start_p: Point<i32> = Point { x: 500, y: 0 };
    let t = vec![
        Vector { x: 0, y: 1 },
        Vector { x: -1, y: 1 },
        Vector { x: 1, y: 1 },
    ];

    for counter in 0.. {
        let mut p: Point<i32> = start_p;

        if map.get(&start_p).is_some() {
            return Some(counter);
        }

        loop {
            if let Some(v) = t.iter().find(|v| map.get(&(p + **v)).is_none()) {
                p = p + *v;
            } else {
                map.entry(p).or_insert('o');
                break;
            }

            if p.y == map_max.y + 1 {
                map.entry(p).or_insert('o');
                break;
            }
        }
        // print_map(
        //     map_min + Vector { x: -10, y: 0 },
        //     map_max + Vector { x: 10, y: 3 },
        //     &map,
        // );
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
