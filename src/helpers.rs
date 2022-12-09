/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */
use rusttype::Vector;
type V = Vector<i32>;

pub static NEIGHBOURS: [V; 8] = [
    V { x: -1, y: -1 },
    V { x: 1, y: -1 },
    V { x: -1, y: 1 },
    V { x: 1, y: 1 },
    V { x: 0, y: -1 },
    V { x: 1, y: 0 },
    V { x: 0, y: 1 },
    V { x: -1, y: 0 },
];

pub static DIRECTIONS: [V; 4] = [
    V { x: 0, y: -1 },
    V { x: 1, y: 0 },
    V { x: 0, y: 1 },
    V { x: -1, y: 0 },
];
