use regex::Regex;
use std::fs;

struct Assignment {
    a1: u32,
    a2: u32,
    b1: u32,
    b2: u32,
}

impl Assignment {
    fn is_inclusion(&self) -> bool {
        return (&self.a1 >= &self.b1 && &self.a2 <= &self.b2)
            || (&self.a1 <= &self.b1 && &self.a2 >= &self.b2);
    }

    fn is_overlapping(&self) -> bool {
        return (&self.a1 >= &self.b1 && &self.a1 <= &self.b2)
            || (&self.b1 >= &self.a1 && &self.b1 <= &self.a2);
    }
}

pub fn main() {
    let assignments: Vec<Assignment> = parse_assignments(read_file());
    println!(
        "DAY 04 | Part 1: {} | Part 2: {}",
        part1(&assignments),
        part2(&assignments)
    );
}

fn read_file() -> String {
    return fs::read_to_string("src/inputs/day04.txt").expect("Input file not readable");
}

fn parse_assignments(data: String) -> Vec<Assignment> {
    let re = Regex::new(r"(^\d*)-(\d*),(\d*)-(\d*)$").unwrap();
    return data
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            return Assignment {
                a1: caps[1].parse().unwrap(),
                a2: caps[2].parse().unwrap(),
                b1: caps[3].parse().unwrap(),
                b2: caps[4].parse().unwrap(),
            };
        })
        .collect();
}

fn part1(items: &Vec<Assignment>) -> usize {
    return items.into_iter().filter(|a| a.is_inclusion()).count();
}

fn part2(items: &Vec<Assignment>) -> usize {
    return items.into_iter().filter(|a| a.is_overlapping()).count();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    fn sample_input() -> Vec<Assignment> {
        return parse_assignments(SAMPLE_INPUT.into());
    }

    #[test]
    fn test_part1_with_sample_input() {
        assert_eq!(part1(&sample_input()), 2);
    }

    #[test]
    fn test_part1_with_day_input() {
        assert_eq!(part1(&parse_assignments(read_file())), 487);
    }

    #[test]
    fn test_part2_with_sample_input() {
        assert_eq!(part2(&sample_input()), 4);
    }

    #[test]
    fn test_part2_with_day_input() {
        assert_eq!(part2(&parse_assignments(read_file())), 849);
    }
}
