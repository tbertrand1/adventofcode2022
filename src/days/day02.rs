use std::fs;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn parse_shape(value: &String) -> &Shape {
    return match value.as_str() {
        "A" | "X" => &Shape::Rock,
        "B" | "Y" => &Shape::Paper,
        "C" | "Z" => &Shape::Scissors,
        _ => panic!("Shape not found"),
    };
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl MatchResult {
    fn score(&self) -> u32 {
        match *self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Loss => 0,
        }
    }
}

fn parse_match_result(value: &String) -> &MatchResult {
    return match value.as_str() {
        "X" => &MatchResult::Loss,
        "Y" => &MatchResult::Draw,
        "Z" => &MatchResult::Win,
        _ => panic!("MatchResult not found"),
    };
}

struct Match {
    value1: String,
    value2: String,
}

impl Match {
    fn other_shape(&self) -> &Shape {
        return parse_shape(&self.value1);
    }

    fn me_shape(&self) -> &Shape {
        return parse_shape(&self.value2);
    }

    fn expected_result(&self) -> &MatchResult {
        return parse_match_result(&self.value2);
    }

    fn score_part1(&self) -> u32 {
        let other_shape: &Shape = self.other_shape();
        let me_shape: &Shape = self.me_shape();
        let match_result: MatchResult = match (other_shape, me_shape) {
            (Shape::Rock, Shape::Rock) => MatchResult::Draw,
            (Shape::Paper, Shape::Rock) => MatchResult::Loss,
            (Shape::Scissors, Shape::Rock) => MatchResult::Win,
            (Shape::Rock, Shape::Paper) => MatchResult::Win,
            (Shape::Paper, Shape::Paper) => MatchResult::Draw,
            (Shape::Scissors, Shape::Paper) => MatchResult::Loss,
            (Shape::Rock, Shape::Scissors) => MatchResult::Loss,
            (Shape::Paper, Shape::Scissors) => MatchResult::Win,
            (Shape::Scissors, Shape::Scissors) => MatchResult::Draw,
        };
        return me_shape.score() + match_result.score();
    }

    fn score_part2(&self) -> u32 {
        let other_shape: &Shape = self.other_shape();
        let expected_result: &MatchResult = self.expected_result();
        let me_shape: Shape = match (other_shape, expected_result) {
            (Shape::Rock, MatchResult::Loss) => Shape::Scissors,
            (Shape::Rock, MatchResult::Draw) => Shape::Rock,
            (Shape::Rock, MatchResult::Win) => Shape::Paper,
            (Shape::Paper, MatchResult::Loss) => Shape::Rock,
            (Shape::Paper, MatchResult::Draw) => Shape::Paper,
            (Shape::Paper, MatchResult::Win) => Shape::Scissors,
            (Shape::Scissors, MatchResult::Loss) => Shape::Paper,
            (Shape::Scissors, MatchResult::Draw) => Shape::Scissors,
            (Shape::Scissors, MatchResult::Win) => Shape::Rock,
        };
        return me_shape.score() + expected_result.score();
    }
}

pub fn main() {
    let matches: Vec<Match> = parse_matches(read_file());
    println!(
        "DAY 02 | Part 1: {} | Part 2: {}",
        part1(&matches),
        part2(&matches)
    );
}

fn read_file() -> String {
    return fs::read_to_string("src/inputs/day02.txt").expect("Input file not readable");
}

fn parse_matches(data: String) -> Vec<Match> {
    return data.lines().map(|l| parse_match(l)).collect();
}

fn parse_match(line: &str) -> Match {
    let shapes: Vec<&str> = line.split(" ").collect();
    return Match {
        value1: shapes[0].into(),
        value2: shapes[1].into(),
    };
}

fn part1(matches: &Vec<Match>) -> u32 {
    return matches.into_iter().map(|m: &Match| m.score_part1()).sum();
}

fn part2(matches: &Vec<Match>) -> u32 {
    return matches.into_iter().map(|m: &Match| m.score_part2()).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "A Y
B X
C Z";

    fn sample_input() -> Vec<Match> {
        return parse_matches(SAMPLE_INPUT.into());
    }

    #[test]
    fn test_part1_with_sample_input() {
        assert_eq!(part1(&sample_input()), 15);
    }

    #[test]
    fn test_part1_with_day_input() {
        assert_eq!(part1(&parse_matches(read_file())), 13682);
    }

    #[test]
    fn test_part2_with_sample_input() {
        assert_eq!(part2(&sample_input()), 12);
    }

    #[test]
    fn test_part2_with_day_input() {
        assert_eq!(part2(&parse_matches(read_file())), 12881);
    }
}
