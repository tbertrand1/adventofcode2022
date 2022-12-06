use std::{collections::HashSet, fs};

pub fn main() {
    let data = read_file();
    println!(
        "DAY 06 | Part 1: {} | Part 2: {}",
        part1(&data),
        part2(&data)
    );
}

fn read_file() -> String {
    return fs::read_to_string("src/inputs/day06.txt").expect("Input file not readable");
}

fn part1(data: &String) -> usize {
    return find_index(data, 4);
}

fn part2(data: &String) -> usize {
    return find_index(data, 14);
}

fn find_index(data: &String, size: usize) -> usize {
    let mut index = size;
    let all_chars: Vec<char> = data.chars().collect();
    for chars in all_chars.windows(size) {
        let unique_chars: HashSet<char> = chars.iter().cloned().collect();
        if unique_chars.len() == chars.len() {
            break;
        }
        index += 1;
    }
    return index;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_sample_input() {
        let data = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(part1(&data), 7);
    }

    #[test]
    fn test_part1_with_day_input() {
        assert_eq!(part1(&read_file()), 1929);
    }

    #[test]
    fn test_part2_with_sample_input() {
        let data = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(part2(&data), 19);
    }

    #[test]
    fn test_part2_with_day_input() {
        assert_eq!(part2(&read_file()), 3298);
    }
}
