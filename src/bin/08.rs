use advent_of_code::helpers::DIRECTIONS;
use rusttype::{Point, Vector};
use std::collections::HashMap;

fn generate_positions_in_direction(
    point: Point<i32>,
    direction: Vector<i32>,
    x_max: i32,
    y_max: i32,
) -> Vec<Point<i32>> {
    let mut points: Vec<Point<i32>> = Vec::new();
    let mut p = point;
    loop {
        p = p + direction;
        points.push(p);
        if p.x == 0 || p.y == 0 || p.x == x_max || p.y == y_max {
            break;
        }
    }
    points
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashMap<(usize, usize), (i32, bool)> = HashMap::new();

    let mut y_max = 0;
    let mut x_max = 0;
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), (c.to_digit(10).unwrap() as i32, false));
            x_max = x_max.max(x);
        }
        y_max = y_max.max(y);
    }

    let mut height: i32;
    for y in 1..y_max {
        height = -1;
        for x in 0..x_max {
            let entry = map.entry((x, y)).or_default();
            if entry.0 > height {
                height = entry.0;
                entry.1 = true;
            }
        }
        height = -1;
        for x in (1..x_max + 1).rev() {
            let entry = map.entry((x, y)).or_default();
            if entry.0 > height {
                height = entry.0;
                entry.1 = true;
            }
        }
    }
    for x in 1..x_max {
        height = -1;
        for y in 0..y_max {
            let entry = map.entry((x, y)).or_default();
            if entry.0 > height {
                height = entry.0;
                entry.1 = true;
            }
        }
        height = -1;
        for y in (1..y_max + 1).rev() {
            let entry = map.entry((x, y)).or_default();
            if entry.0 > height {
                height = entry.0;
                entry.1 = true;
            }
        }
    }

    let count: u32 = map.iter().map(|(_, v)| u32::from(v.1)).sum();
    Some(count + 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<(usize, usize), (i32, bool)> = HashMap::new();

    let mut y_max = 0;
    let mut x_max = 0;
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), (c.to_digit(10).unwrap() as i32, false));
            x_max = x_max.max(x);
        }
        y_max = y_max.max(y);
    }

    let mut highest = 0;
    for y in 1..y_max {
        for x in 1..x_max {
            let mut score = 1;
            let (height, _) = map.get(&(x, y)).unwrap();
            for dir in DIRECTIONS {
                let mut dir_score = 0;
                for p in generate_positions_in_direction(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    dir,
                    x_max as i32,
                    y_max as i32,
                ) {
                    let (v, _) = map.get(&(p.x as usize, p.y as usize)).unwrap();
                    dir_score += 1;
                    if v >= height {
                        break;
                    }
                }
                score *= dir_score;
            }
            if score > highest {
                highest = score;
            }
        }
    }

    Some(highest)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
