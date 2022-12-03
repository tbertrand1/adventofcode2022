use std::fs;

const UPPER_A_ASCII_VALUE: u32 = 65;
const LOWER_A_ASCII_VALUE: u32 = 97;

pub fn main() {
    let items: Vec<String> = read_file();
    println!(
        "DAY 03 | Part 1: {} | Part 2: {}",
        part1(&items),
        part2(&items)
    );
}

fn read_file() -> Vec<String> {
    return fs::read_to_string("src/inputs/day03.txt")
        .expect("Input file not readable")
        .lines()
        .map(|l| l.into())
        .collect();
}

fn part1(items: &Vec<String>) -> u32 {
    return items
        .into_iter()
        .map(|line| calculate_priority_part1(&line))
        .sum();
}

fn calculate_priority_part1(line: &String) -> u32 {
    let compartments: (&str, &str) = line.split_at(line.len() / 2);
    for c in compartments.0.chars() {
        if compartments.1.contains(c) {
            return priority_value(c);
        }
    }
    panic!("No common character found");
}

fn part2(items: &Vec<String>) -> u32 {
    return items
        .chunks(3)
        .map(|rucksacks| calculate_priority_part2(&rucksacks))
        .sum();
}

fn calculate_priority_part2(lines: &[String]) -> u32 {
    for c in lines[0].chars() {
        if lines[1].contains(c) && lines[2].contains(c) {
            return priority_value(c);
        }
    }
    panic!("No common character found");
}

fn priority_value(c: char) -> u32 {
    let ascii_value: u32 = c as u32;
    if ascii_value >= LOWER_A_ASCII_VALUE {
        return ascii_value - LOWER_A_ASCII_VALUE + 1;
    }
    return ascii_value - UPPER_A_ASCII_VALUE + 1 + 26;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> [String; 6] {
        return [
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];
    }

    #[test]
    fn test_part1_with_sample_input() {
        assert_eq!(part1(&sample_input().to_vec()), 157);
    }

    #[test]
    fn test_part1_with_day_input() {
        assert_eq!(part1(&read_file()), 8105);
    }

    #[test]
    fn test_part2_with_sample_input() {
        assert_eq!(part2(&sample_input().to_vec()), 70);
    }

    #[test]
    fn test_part2_with_day_input() {
        assert_eq!(part2(&read_file()), 2363);
    }
}
