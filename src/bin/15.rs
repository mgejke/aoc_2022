use itertools::Itertools;
use rusttype::Point;

#[macro_use]
extern crate scan_fmt;

use std::cmp;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Range {
        Range { start, end }
    }

    fn overlaps(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }

    fn merge(&mut self, other: &Range) {
        self.start = cmp::min(self.start, other.start);
        self.end = cmp::max(self.end, other.end);
    }
}

#[derive(Debug, Clone)]
struct RangeStack {
    ranges: Vec<Range>,
}

impl RangeStack {
    fn add(&mut self, range: &Range) {
        if let Some(last) = self.ranges.last_mut() {
            if last.overlaps(range) {
                last.merge(range);
                return;
            }
        }
        self.ranges.push(*range);
    }
}

impl FromIterator<Range> for RangeStack {
    fn from_iter<I>(iterator: I) -> Self
    where
        I: IntoIterator<Item = Range>,
    {
        let mut raw_ranges: Vec<_> = iterator.into_iter().collect();
        raw_ranges.sort_by(|a, b| a.start.cmp(&b.start));

        let mut range_stack = RangeStack { ranges: Vec::new() };

        for range in &raw_ranges {
            range_stack.add(range);
        }

        range_stack
    }
}

struct Sensor {
    p: Point<i32>,
    r: i32,
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            match scan_fmt!(
                line,
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                i32,
                i32,
                i32,
                i32
            ) {
                Ok((x1, y1, x2, y2)) => Some(Sensor {
                    p: Point { x: x1, y: y1 },
                    r: (x1 - x2).abs() + (y1 - y2).abs(),
                }),
                _ => None,
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let c = parse_sensors(input);

    let row = 2000000;
    let mut isnt: Vec<Range> = vec![];

    for s in &c {
        let distance_to_row = (s.p.y - row).abs();
        if distance_to_row < s.r {
            isnt.push(Range::new(
                s.p.x - (s.r - distance_to_row),
                s.p.x + (s.r - distance_to_row),
            ));
        }
    }

    let stack = RangeStack::from_iter(isnt);
    let v = stack
        .ranges
        .iter()
        .fold(0, |acc, v| acc + (v.end - v.start));

    Some(v as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let c = parse_sensors(input);

    let size = 4_000_000;
    for row in 0..=size {
        let mut isnt: Vec<Range> = vec![];

        for s in &c {
            let distance_to_row = (s.p.y - row).abs();
            if distance_to_row < s.r {
                isnt.push(Range::new(
                    s.p.x - (s.r - distance_to_row),
                    s.p.x + (s.r - distance_to_row),
                ));
            }
        }

        let stack = RangeStack::from_iter(isnt);
        if stack.ranges.len() > 1 {
            return Some(4_000_000 * stack.ranges.first().unwrap().end as u64 + row as u64);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
