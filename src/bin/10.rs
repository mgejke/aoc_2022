use std::collections::VecDeque;

#[derive(Clone)]
enum Instruction {
    Nop,
    Addx(i32),
}

fn parse_instructions(input: &str) -> VecDeque<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (instruction, param) = line.split_once(' ').unwrap_or((line, ""));
            match instruction {
                "addx" => Instruction::Addx(param.parse::<i32>().unwrap()),
                "noop" => Instruction::Nop,
                _ => panic!("Illegal instruction"),
            }
        })
        .collect::<VecDeque<Instruction>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut instructions = parse_instructions(input.trim());
    let mut state: i32 = 1;
    let mut signal: i32 = 0;
    let mut execution_slot: (u32, Instruction) = (0, Instruction::Nop);
    for c in 0..500 {
        if (c - 20) % 40 == 0 {
            signal += c * state;
        }

        if execution_slot.0 == 0 {
            match execution_slot.1 {
                Instruction::Addx(n) => state += n,
                Instruction::Nop => (),
            }

            if let Some(instruction) = instructions.pop_front() {
                match instruction {
                    Instruction::Addx(_) => execution_slot = (1, instruction),
                    Instruction::Nop => execution_slot = (0, instruction),
                }
            } else {
                break;
            }
        } else {
            execution_slot.0 -= 1;
        }
    }
    Some(signal as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut instructions = parse_instructions(input.trim());
    let mut state: i32 = 1;
    let mut execution_slot: (u32, Instruction) = (0, Instruction::Nop);
    for c in 0..500 {
        if execution_slot.0 == 0 {
            match execution_slot.1 {
                Instruction::Addx(n) => state += n,
                Instruction::Nop => (),
            }

            if let Some(instruction) = instructions.pop_front() {
                match instruction {
                    Instruction::Addx(_) => execution_slot = (1, instruction),
                    Instruction::Nop => execution_slot = (0, instruction),
                }
            } else {
                break;
            }
        } else {
            execution_slot.0 -= 1;
        }
        if c % 40 == 0 {
            println!();
        }
        let screen = if ((c % 40) - state).abs() > 1 {
            '.'
        } else {
            '#'
        };
        print!("{screen}");
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
