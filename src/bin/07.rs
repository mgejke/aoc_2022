use std::{collections::HashMap, path::PathBuf, str::FromStr};

enum Entry {
    Folder(PathBuf),
    File(usize),
}

fn get_size(fs: &HashMap<PathBuf, Vec<Entry>>, path: &PathBuf) -> usize {
    fs[path]
        .iter()
        .map(|e| match e {
            Entry::Folder(path) => get_size(fs, path),
            Entry::File(size) => *size,
        })
        .sum()
}

fn get_fs(input: &str) -> HashMap<PathBuf, Vec<Entry>> {
    let mut fs: HashMap<PathBuf, Vec<Entry>> = HashMap::new();
    let mut current = PathBuf::from_str("/").unwrap();
    for line in input.trim().lines().skip(1) {
        let mut row = line.split_whitespace();
        let command_or_other = row.next().unwrap();
        if command_or_other == "$" {
            match row.next().unwrap() {
                "cd" => {
                    let f = row.next().unwrap();
                    if f == ".." {
                        current.pop();
                    } else {
                        current.push(f);
                    }
                }
                "ls" => (),
                _ => panic!("Unknown command"),
            }
        } else if let Ok(size) = command_or_other.parse::<usize>() {
            fs.entry(current.clone())
                .or_default()
                .push(Entry::File(size))
        } else {
            let dir_name = row.next().unwrap();
            let new_dir = current.join(dir_name);
            fs.entry(new_dircar).or_default();
            fs.entry(current.clone())
                .or_default()
                .push(Entry::Folder(new_dir.clone()));
        }
    }
    fs
}

pub fn part_one(input: &str) -> Option<usize> {
    let fs = get_fs(input);

    let size: usize = fs
        .iter()
        .map(|(k, _)| get_size(&fs, k))
        .filter(|v| *v < 100000)
        .sum();

    Some(size)
}

pub fn part_two(input: &str) -> Option<usize> {
    let fs = get_fs(input);

    let free = 70000000 - get_size(&fs, &PathBuf::from_str("/").unwrap());

    let larger = fs
        .iter()
        .map(|(k, _)| get_size(&fs, k))
        .filter(|v| *v > 30000000 - free)
        .fold(usize::MAX, |acc, v| acc.min(v));

    Some(larger)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
