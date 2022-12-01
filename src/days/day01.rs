use std::fs;

pub fn main() {
    let calories: Vec<u32> = parse_and_sort_calories(read_file());
    println!(
        "DAY 01 | Part 1: {} | Part 2: {}",
        calories[0],
        (calories[0] + calories[1] + calories[2])
    );
}

fn read_file() -> String {
    return fs::read_to_string("src/inputs/day01.txt").expect("Input file not readable");
}

fn parse_and_sort_calories(data: String) -> Vec<u32> {
    let mut calories: Vec<u32> = data
        .split("\n\n")
        .map(|cs| cs.split("\n").map(|c| c.parse::<u32>().unwrap()).sum())
        .collect();
    calories.sort_by(|a, b| b.cmp(a));
    return calories;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_parse_and_sort_calories_with_sample_input() {
        let calories: Vec<u32> = parse_and_sort_calories(SAMPLE_INPUT.into());
        assert_eq!(calories[0], 24000);
        assert_eq!(calories[0] + calories[1] + calories[2], 45000);
    }

    #[test]
    fn test_parse_and_sort_calories_with_day_input() {
        let calories: Vec<u32> = parse_and_sort_calories(read_file());
        assert_eq!(calories[0], 70296);
        assert_eq!(calories[0] + calories[1] + calories[2], 205381);
    }
}
