use regex::Regex;
use std::fs;

struct Step {
    size: u32,
    stack_from: u32,
    stack_to: u32,
}

pub fn main() {
    let (stacks, steps): (Vec<Vec<char>>, Vec<Step>) = parse_data(read_file());

    println!(
        "DAY 05 | Part 1: {} | Part 2: {}",
        part1(&mut stacks.to_vec(), &steps),
        part2(&mut stacks.to_vec(), &steps)
    );
}

fn read_file() -> String {
    return fs::read_to_string("src/inputs/day05.txt").expect("Input file not readable");
}

fn parse_data(data: String) -> (Vec<Vec<char>>, Vec<Step>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut steps: Vec<Step> = Vec::new();

    let step_regex: Regex = Regex::new(r"^move (\d*) from (\d*) to (\d**)$").unwrap();
    let mut stacks_part: bool = true;
    let mut nb_stacks: u32 = 0;
    for line in data.lines() {
        if nb_stacks == 0 {
            nb_stacks = ((line.len() + 1) / 4) as u32;
            for _ in 0..nb_stacks {
                stacks.push(Vec::new())
            }
        }
        if line.len() == 0 {
            stacks_part = false;
            continue;
        }

        if stacks_part {
            if line.contains("[") {
                for index in 0..nb_stacks {
                    let char_index = 4 * index + 1;
                    let char: char = line.chars().nth(char_index as usize).unwrap();
                    if char != ' ' {
                        stacks[index as usize].push(char);
                    }
                }
            }
        } else {
            let caps = step_regex.captures(line).unwrap();
            steps.push(Step {
                size: caps[1].parse().unwrap(),
                stack_from: caps[2].parse().unwrap(),
                stack_to: caps[3].parse().unwrap(),
            });
        }
    }
    return (stacks, steps);
}

fn part1(stacks: &mut Vec<Vec<char>>, steps: &Vec<Step>) -> String {
    apply_steps_part1(stacks, steps);
    return get_first_of_stacks(stacks);
}

fn part2(stacks: &mut Vec<Vec<char>>, steps: &Vec<Step>) -> String {
    apply_steps_part2(stacks, steps);
    return get_first_of_stacks(stacks);
}

fn apply_steps_part1(stacks: &mut Vec<Vec<char>>, steps: &Vec<Step>) {
    for step in steps {
        for _ in 0..step.size {
            let stack_from_index = (step.stack_from - 1) as usize;
            let stack_to_index = (step.stack_to - 1) as usize;
            let char = stacks[stack_from_index].remove(0);
            stacks[stack_to_index].insert(0, char);
        }
    }
}

fn apply_steps_part2(stacks: &mut Vec<Vec<char>>, steps: &Vec<Step>) {
    for step in steps {
        for index in (0..step.size).rev() {
            let stack_from_index = (step.stack_from - 1) as usize;
            let stack_to_index = (step.stack_to - 1) as usize;
            let char = stacks[stack_from_index].remove(index as usize);
            stacks[stack_to_index].insert(0, char);
        }
    }
}

fn get_first_of_stacks(stacks: &mut Vec<Vec<char>>) -> String {
    return stacks
        .iter()
        .map(|stack| stack.first().unwrap())
        .collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> String {
        return fs::read_to_string("src/inputs/day05-sample.txt").expect("Input file not readable");
    }

    #[test]
    fn test_part1_with_sample_input() {
        let (mut stacks, steps) = parse_data(sample_input());
        assert_eq!(part1(&mut stacks, &steps), "CMZ");
    }

    #[test]
    fn test_part1_with_day_input() {
        let (mut stacks, steps) = parse_data(read_file());
        assert_eq!(part1(&mut stacks, &steps), "BWNCQRMDB");
    }

    #[test]
    fn test_part2_with_sample_input() {
        let (mut stacks, steps) = parse_data(sample_input());
        assert_eq!(part2(&mut stacks, &steps), "MCD");
    }

    #[test]
    fn test_part2_with_day_input() {
        let (mut stacks, steps) = parse_data(read_file());
        assert_eq!(part2(&mut stacks, &steps), "NHWZCBNBF");
    }
}
